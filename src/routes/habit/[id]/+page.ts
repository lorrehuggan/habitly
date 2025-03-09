import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "../../$types";
import type { Habit } from "../../../types/timeline";

export const load: PageLoad = async ({ params }) => {
  try {
    const habit: Habit = await invoke("get_habit_by_id", { id: params.id });
    console.log(habit);
    return {
      habit,
    };
  } catch (e) {
    console.log(e);
  }
};
