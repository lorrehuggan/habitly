import type { Timeline } from "../types/timeline";
import type { PageLoad } from "./$types";
import { invoke } from "@tauri-apps/api/core";

export const load: PageLoad = async () => {
  const timeline: Timeline = await invoke("init_timeline");

  return timeline;
};
