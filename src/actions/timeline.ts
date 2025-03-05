import { invoke } from "@tauri-apps/api/core";
import type { Commit } from "../types/timeline";

export async function actionGetCommits(id: string) {
  try {
    const response: Commit[] = await invoke("get_habit_commits", { id });
    return response;
  } catch (e) {
    //TODO: handle error
    console.log(e);
  }
}
