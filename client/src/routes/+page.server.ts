import type { PageServerLoad } from "./login/$types";

export const load: PageServerLoad = async () => {
    const res = await fetch("http://127.0.0.1:8080/api/projects/all");
    const projects: ProjectResponse[] = await res.json();

    return { projects };
};