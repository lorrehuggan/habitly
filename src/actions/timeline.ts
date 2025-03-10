import { invoke } from "@tauri-apps/api/core";
import type { Commit } from "../types/timeline";
import { goto } from "$app/navigation";

export async function getCommits(id: string) {
  try {
    const response: Commit[] = await invoke("get_habit_commits", { id });
    return response;
  } catch (e) {
    //TODO: handle error
    console.log(e);
  }
}

export async function deleteHabit(id: string) {
  try {
    const response: { message: string; error: boolean } = await invoke("delete_habit", { id });

    if (response.message === "success") {
      goto("/");
    }
  } catch (e) {
    //TODO: handle error
    console.log(e);
  }
}

export async function archiveHabit(id: string) {
  try {
    const response: { message: string; error: boolean } = await invoke("archive_habit", { id });

    if (response.message === "success") {
      goto("/");
    }
  } catch (e) {
    //TODO: handle error
    console.log(e);
  }
}

export async function restoreArchivedHabit(id: string) {
  try {
    const response: { message: string; error: boolean } = await invoke("restore_habit", { id });

    if (response.message === "success") {
      goto("/");
    }
  } catch (e) {
    //TODO: handle error
    console.log(e);
  }
}
