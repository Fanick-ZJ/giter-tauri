import { STATUS_CHANGE } from "@/const/listen";
import { parseStatus, RepoStatus, SetupStoreId } from "@/enum";
import { Repository } from "@/types";
import { addWatch, isRepo, removeWatch, workStatus } from "@/utils/command";
import { readRepos, removeRepo, saveRepo, updateRepo } from "@/utils/store";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { QueryResult } from "@tauri-apps/plugin-sql";
import { defineStore } from "pinia";
import { ref, Ref } from "vue";

type RepoPath = string

export type ValidRepository = Repository & { valid: boolean; loading: boolean }

type StatusChangeCallback = (path: string, status: RepoStatus) => void

/** 从 unknown 错误中提取可读消息 */
function getErrorMessage(err: unknown): string {
  return err instanceof Error ? err.message : "未知错误";
}

export const useRepoStore = defineStore(SetupStoreId.Repo, () => {
  const repos = ref<ValidRepository[]>([]);
  const status: Map<RepoPath, Ref<RepoStatus>> = new Map();
  const statusChangeCbs: StatusChangeCallback[] = [];

  // ==================== 工具函数 ====================

  const validateRepo = async (path: string): Promise<boolean> => {
    try {
      return await isRepo(path);
    } catch (err: unknown) {
      window.$message.warning(
        `仓库路径 ${path} 验证异常: ${getErrorMessage(err)}, 默认标记为无效`,
      );
      return false;
    }
  };

  const safeAddWatch = async (path: string): Promise<boolean> => {
    try {
      await addWatch(path);
      return true;
    } catch (err: unknown) {
      window.$message.warning(`仓库 ${path} 监听添加失败: ${getErrorMessage(err)}`);
      return false;
    }
  };

  const manageRepoStatus = (path: RepoPath): Ref<RepoStatus> => {
    const statusRef = ref<RepoStatus>(RepoStatus.Ok);
    status.set(path, statusRef);
    return statusRef;
  };

  /** 仓库初始化核心逻辑：验证、监听、获取状态 */
  const commonRepoInit = async (repo: Repository): Promise<ValidRepository> => {
    const validRepo: ValidRepository = { ...repo, valid: true, loading: true };

    validRepo.valid = await validateRepo(validRepo.path);
    if (!validRepo.valid) return validRepo;

    void safeAddWatch(validRepo.path);

    const statusRef = manageRepoStatus(validRepo.path);

    try {
      const rawStatus = await workStatus(validRepo.path);
      if (parseStatus(rawStatus).length > 0) {
        statusRef.value = rawStatus as RepoStatus;
      } else {
        throw new Error(`未知仓库状态: ${rawStatus}`);
      }
    } catch (err: unknown) {
      validRepo.valid = false;
      window.$message.error(`获取仓库状态失败: ${getErrorMessage(err)}`);
    }

    return validRepo;
  };

  // ==================== 排序 ====================

  const repoSort = (a: Repository, b: Repository): number => {
    // top 降序（true > false）
    if (a.top !== b.top) return a.top ? -1 : 1;
    // order 升序
    if (a.order !== b.order) return a.order - b.order;
    // alias 升序
    return a.alias.localeCompare(b.alias);
  };

  const defaultSort = (): void => {
    repos.value = [...repos.value].sort(repoSort);
  };

  const sortByStatus = (): void => {
    repos.value = [...repos.value].sort((a, b) => {
      const aStatus = status.get(a.path)?.value ?? RepoStatus.Ok;
      const bStatus = status.get(b.path)?.value ?? RepoStatus.Ok;
      // 状态降序
      if (aStatus !== bStatus) return bStatus - aStatus;
      return repoSort(a, b);
    });
  };

  // ==================== 仓库 CRUD ====================

  const init_repo = async (): Promise<void> => {
    const storedRepos = await readRepos();
    const initialRepos: ValidRepository[] = storedRepos.map((repo) => ({
      ...repo,
      hasWatch: !!repo.hasWatch,
      top: !!repo.top,
      valid: true,
      loading: true,
    }));
    repos.value = [...initialRepos].sort(repoSort);

    // 并行初始化所有仓库，使用 id 匹配更新避免竞态
    const initPromises = initialRepos.map(async (repo) => {
      const result = await commonRepoInit(repo);
      repos.value = repos.value.map((r) =>
        r.id === result.id ? { ...result, loading: false } : r,
      );
      return result;
    });

    await Promise.all(initPromises);
  };

  const add = async (repo: Repository): Promise<void> => {
    try {
      const res: QueryResult = await saveRepo(repo);
      repo.id = res.lastInsertId as number;
      const result = await commonRepoInit(repo);
      repos.value = [...repos.value, result].sort(repoSort);
    } catch (err: unknown) {
      window.$message.error(`添加仓库失败: ${getErrorMessage(err)}`);
    }
  };

  const update = async (repo: Repository): Promise<void> => {
    await updateRepo(repo);
    const index = repos.value.findIndex((r) => r.id === repo.id);
    if (index === -1) return;
    repos.value = [
      ...repos.value.slice(0, index),
      { ...repos.value[index], ...repo },
      ...repos.value.slice(index + 1),
    ].sort(repoSort);
  };

  const remove = async (repo: ValidRepository): Promise<void> => {
    await removeRepo(repo.id);
    void removeWatch(repo.path);
    repos.value = repos.value.filter((r) => r.id !== repo.id);
  };

  // ==================== 查询 ====================

  const getRepoById = (id: number): ValidRepository | undefined => {
    return repos.value.find((repo) => repo.id === id);
  };

  const getRepoByPath = (path: RepoPath): ValidRepository | undefined => {
    return repos.value.find((repo) => repo.path === path);
  };

  // ==================== 状态管理 ====================

  const setStatus = (path: RepoPath, newStatus: RepoStatus): void => {
    const ref = status.get(path);
    if (ref) ref.value = newStatus;
  };

  /** 注册状态变化回调，返回取消注册的函数 */
  const addStatusChangeCb = (cb: StatusChangeCallback): (() => void) => {
    statusChangeCbs.push(cb);
    return () => {
      const index = statusChangeCbs.indexOf(cb);
      if (index !== -1) {
        statusChangeCbs.splice(index, 1);
      }
    };
  };

  // ==================== 事件监听生命周期 ====================

  let unlisten: UnlistenFn | null = null;

  const startListening = async (): Promise<void> => {
    if (unlisten) return;
    unlisten = await listen<{ path: string; status: RepoStatus }>(
      STATUS_CHANGE,
      (event) => {
        const { path, status: repoStatus } = event.payload;
        setStatus(path, repoStatus);
        statusChangeCbs.forEach((cb) => cb(path, repoStatus));
      },
    );
  };

  const stopListening = (): void => {
    unlisten?.();
    unlisten = null;
  };

  // 自动启动监听
  void startListening();

  return {
    // state
    repos,
    status,
    // init
    init_repo,
    // mutations
    add,
    update,
    remove,
    // queries
    getRepoById,
    getRepoByPath,
    // sorting
    defaultSort,
    sortByStatus,
    // lifecycle
    addStatusChangeCb,
    startListening,
    stopListening,
  };
});
