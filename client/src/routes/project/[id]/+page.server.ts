import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const res = await fetch("http://127.0.0.1:8080/api/projects/unique", {
        method: "post",
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(params.id)
    });
    
    if (!res.ok) return { err: await res.text() };

    const project = await res.json() as ProjectResponse;
    return { project };
};