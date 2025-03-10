import type { Habit, UserSettings, Timeline } from "../../types/timeline";
import type { PageLoad } from "./$types";
import { invoke } from "@tauri-apps/api/core";

export const load: PageLoad = async () => {
  try {
    const habits: Habit[] = await invoke("get_archived_habits");
    return { habits };
  } catch (e) {
    console.log(e);
    return { error: e };
  }
};
