import type { PageServerLoad } from "./login/$types";

export const load: PageServerLoad = async () => {
    const res = await fetch("http://127.0.0.1:8080/api/projects/all");

    if (!res.ok) {
       return { err: await res.text() } 
    }
    const projects: ProjectResponse[] = await res.json();

    return { projects };
};