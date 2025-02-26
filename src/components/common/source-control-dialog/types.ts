import { ValidRepository } from "@/store/modules/repo"

export type SourceConterolDialogProps = {
  repo: ValidRepository
}

export enum FileStatus {
  ADDED = 'added',
  MODIFIED = 'modified',
  DELETED = 'deleted',
}