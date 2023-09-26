import type { Bus } from "$lib/types";
import { invoke } from "@tauri-apps/api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  let bus: Bus = await invoke("get_bus_now");

  return {
    props: bus,
  };
};
