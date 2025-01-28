import { CommandError } from "@/enum/error"

export type Repository = {
  id: number
  path: string
  alias: string
  hasWatch: boolean
  order: number
  top: boolean
}

export type Error = {
  type: keyof typeof CommandError,
  data: string
}