import { CommitFile } from "@/types";
import { getBlobContent } from "@/utils/command";
import { fileExtension } from "@/utils/tool";
import { NImage } from "naive-ui";
import { Component, StyleValue } from "vue";

type BinaryComponent = {
  name: Component | String,
  param: any,
}
export type BinaryResult = [BinaryComponent | undefined, BinaryComponent | undefined]

const isImage = (file: CommitFile) => {
  const imageTypes = ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'svg', 'webp']
  const fileName = file.path.split('/').pop() || ''
  const fileType = fileName.split('.').pop()
  return imageTypes.includes(fileType?.toLowerCase() || '')
}

const readFile = (repo: string,  cid: String) => {
  return getBlobContent(repo, cid).then((res) => {
    return new Uint8Array(res);
  })
} 

const processImage  = (repo: string,  file: CommitFile) => {
  const result: BinaryResult = [undefined, undefined]
  let _resolve: (res: BinaryResult) => void
  let _reject: (err: any) => void
  const promise = new Promise<BinaryResult>((resolve, reject) => {
    _resolve = resolve
    _reject = reject 
  })
  if (file.status === 'Deleted') {
    readFile(repo, file.prevObjectId).then((res) => {
      const url = URL.createObjectURL(new Blob([res], { type: fileExtension(file.path) }))
      result[1] = {
       name: NImage,
       param: {
        'preview-disabled': true,
        src: url,
       }, 
      }
      _resolve(result)
    }).catch((err) => {
      _reject(err) 
    })
  }
  if (file.status === 'Added') {
    readFile(repo, file.objectId).then((res) => {
      const url = URL.createObjectURL(new Blob([res], { type: fileExtension(file.path) }))
      result[0] = {
       name: NImage,
       param: {
        'preview-disabled': true,
        src: url,
       }
      }
      _resolve(result)
    }).catch((err) => {
      _reject(err)
    })
  }
  if (file.status === 'Modified') {
    const old = readFile(repo, file.prevObjectId)
    const newFile = readFile(repo, file.objectId)
    Promise.allSettled([old, newFile]).then((res) => {
     if (res[0].status === 'fulfilled') {
        const url = URL.createObjectURL(new Blob([res[0].value], { type: fileExtension(file.path) }))
        result[0] = {
         name: NImage,
         param: {
          'preview-disabled': true,
          src: url,
         }
        }
      } 
      if (res[1].status === 'fulfilled') {
        const url = URL.createObjectURL(new Blob([res[1].value], { type: fileExtension(file.path) }))
        result[1] = {
         name: NImage,
         param: {
          'preview-disabled': true,
          src: url,
         }
        }
      }
      _resolve(result)
    }).catch((err) => {
      _reject(err)
    })
  }

  return promise
}

export const processBinaryData = (repo: string,  file: CommitFile) => {
  if (isImage(file)) {
    return processImage(repo, file)
  }
}