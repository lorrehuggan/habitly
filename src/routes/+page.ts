import type { Habit, UserSettings } from "../types/timeline";
import type { Timeline } from "../types/timeline";
import type { PageLoad } from "./$types";
import { invoke } from "@tauri-apps/api/core";

export const load: PageLoad = async () => {
  try {
    const timeline: Timeline = await invoke("init_timeline");
    const habits: Habit[] = await invoke("get_all_habits");
    const userSettings: UserSettings = await invoke("get_settings");
    return { timeline, habits, userSettings };
  } catch (e) {
    console.log(e);
    return { error: e };
  }
};
