import { get_config_db } from "./storage"

export const get_config_value = async (key: string) => {
  return get_config_db().then( async db => {
    let rows = await db.select<[{value: string}]>("SELECT value FROM config WHERE key = $1", [key])
    if (rows.length > 0) {
      return rows[0].value
    } else {
      return null
    }
  })
}

export const set_config_value = async (key: string, value: string) => {
  return get_config_db().then( async db => {
    let rows = await db.select<[{value: string}]>("SELECT value FROM config WHERE key = $1", [key])
    if (rows.length > 0) {
      await db.execute("UPDATE config SET value = $1 WHERE key = $2", [value, key])
    } else {
      await db.execute("INSERT INTO config (key, value) VALUES ($1, $2)", [key, value])
    }
  })
}