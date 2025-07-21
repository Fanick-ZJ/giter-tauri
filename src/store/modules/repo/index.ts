import { STATUS_CHANGE } from "@/const/listen";
import { parseStatus, RepoStatus, SetupStoreId } from "@/enum";
import { Repository } from "@/types";
import { addWatch, isRepo, removeWatch, workStatus } from "@/utils/command";
import { readRepos, removeRepo, saveRepo, updateRepo } from "@/utils/store";
import { listen } from "@tauri-apps/api/event";
import { QueryResult } from "@tauri-apps/plugin-sql";
import { defineStore } from "pinia";
import { Ref, ref, toRaw } from "vue";

type RepoPath = string
export type ValidRepository = Repository & { valid: boolean; loading: boolean }
export const useRepoStore = defineStore(SetupStoreId.Repo, () => {
  const repos = ref<ValidRepository[]>([])
  const status: Map<RepoPath, Ref<RepoStatus>> = new Map()

  // 工具函数：验证仓库有效性（独立可复用，带明确类型）
  const validateRepo = async (path: string): Promise<boolean> => {
    try { return await isRepo(path) } catch (err) {
      window.$message.warning(`仓库路径 ${path} 验证异常: ${err instanceof Error ? err.message : '未知错误'}, 默认标记为无效`);
      return false;
    }
  }

  // 工具函数：安全添加仓库监听（带错误捕获和返回类型）
  const safeAddWatch = async (path: string): Promise<boolean> => {
    try {
      await addWatch(path);
      return true;
    } catch (err) {
      window.$message.warning(`仓库 ${path} 监听添加失败: ${err instanceof Error ? err.message : '未知错误'}`);
      return false;
    }
  }

  // 工具函数：管理仓库状态（集中处理状态存储和更新）
  const manageRepoStatus = (path: RepoPath): Ref<RepoStatus> => {
    const statusRef = ref<RepoStatus>(RepoStatus.Ok);
    status.set(path, statusRef);
    return statusRef;
  }

  // 统一仓库初始化核心逻辑（严格类型+模块化调用）
  const commonRepoInit = async (repo: Repository): Promise<ValidRepository> => {
    const validRepo: ValidRepository = { ...repo, valid: true, loading: true };
    // 1. 验证仓库有效性（带详细错误提示）
    validRepo.valid = await validateRepo(validRepo.path);
    if (!validRepo.valid) return validRepo;

    // 2. 安全添加监听（异步非阻塞，不影响主流程）
    void safeAddWatch(validRepo.path); // 使用void忽略非关键操作的Promise

    // 3. 初始化响应式状态（通过工具函数集中管理）
    const statusRef = manageRepoStatus(validRepo.path);

    // 4. 异步获取状态（添加类型守卫确保类型安全）
    try {
      const rawStatus = await workStatus(validRepo.path);
      // 这里rawStaus是通过位运算出来的，不能简单地判断是否在RepoStatus中，进行修改
      if (parseStatus(rawStatus).length > 0) {
        statusRef.value = rawStatus as RepoStatus;
      } else {
        throw new Error(`未知仓库状态: ${rawStatus}`);
      }
    } catch (err: any) {
      validRepo.valid = false
      window.$message.error(`获取仓库状态失败: ${err.message}`)
    }
    return validRepo;
  }

  // 优化后的初始化逻辑（并行处理+结果过滤）
  const _init_opt = (repo: ValidRepository) => commonRepoInit(repo);

  const repoSort = (a: Repository, b: Repository) => {
    // 按 top 降序
    if (a.top < b.top) return 1;
    if (a.top > b.top) return -1;
  
    // 如果 top 相同，则按 order 升序排序
    if (a.order < b.order) return -1;
    if (a.order > b.order) return 1;
  
    // 如果 top 和 order 都相同，则按 alias 升序排序
    if (a.alias < b.alias) return -1;
    if (a.alias > b.alias) return 1;
  
    // 如果所有属性都相同，返回 0
    return 0;
  }

  const defaultSort = () => {
    repos.value = repos.value.sort(repoSort)
  }

  const sortByStatus = () => {
    repos.value = repos.value.sort((a, b) => {
      // 按状态排序
      const aStatus = status.get(a.path)?.value || RepoStatus.Ok
      const bStatus = status.get(b.path)?.value || RepoStatus.Ok
      if (aStatus < bStatus) return 1
      else if (aStatus > bStatus) return -1
      else return repoSort(a, b)
    })
  }

  const init_repo = async () => {
    const __repos = await readRepos()
    // 1. 立即初始化空列表并标记加载状态，避免白屏
    const initialRepos = __repos.map(repo => ({...repo, hasWatch: !!repo.hasWatch, top: !!repo.top, valid: true, loading: true}))
    repos.value = initialRepos.sort(repoSort)
    // 2. 并行执行初始化（不阻塞UI渲染）
    const initPromises = initialRepos.map(async (repo, index) => {
      const result = await _init_opt(repo)
      // 3. 逐个更新状态，触发响应式更新
      repos.value[index] = {...result, loading: false}
      return result
    })

    // 4. 等待所有初始化完成（可选，用于后续操作）
    await Promise.all(initPromises)
  }

  // 添加仓库
  const add = (repo: Repository) => {
    saveRepo(repo).then((res: QueryResult) => {
      repo.id = res.lastInsertId as number
      // @ts-ignore
      _init_opt(repo).then((res) => {
        repos.value.push(res)
        repos.value.sort(repoSort) 
      })
    })
  }

  // 监听仓库状态变化
  listen(STATUS_CHANGE, (event) => {
    const { path, status } = event.payload as { path: string, status: RepoStatus }
    console.log('status change', path, status)
    setStatus(path, status)
  })


  // 设置仓库状态
  const setStatus = (path: RepoPath, _status: RepoStatus) => {
    if (status.has(path)) {
      status.get(path)!.value = _status
    }
  }

  // 更新仓库信息
  const update = async (repo: Repository) => {
    updateRepo(repo)
    const index = repos.value.findIndex((r) => r.id === repo.id)
    if (index == -1) return
    Object.assign(repos.value[index], repo)
    repos.value.sort(repoSort)
  }

  const getRepoById = (id: number) => {
    return repos.value.find((repo) => repo.id === id) 
  }

  const getRepoByPath = (path: RepoPath) => {
    return repos.value.find((repo) => repo.path === path) 
  }

  const remove = (repo: ValidRepository) => {
    removeRepo(repo.id)
    removeWatch(repo.path)
    const index = repos.value.findIndex((r) => r.id === repo.id)
    if (index == -1) return
    repos.value.splice(index, 1)
  }

  return {
    init_repo,
    repos,
    add,
    status,
    update,
    getRepoByPath,
    getRepoById,
    remove,
    sortByStatus,
    defaultSort
  }
})