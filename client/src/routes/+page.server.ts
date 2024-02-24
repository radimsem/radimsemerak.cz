import type { PageServerLoad } from "./login/$types";

export const load: PageServerLoad = async () => {
    const body: ObfuscRequest = {
        content: "semerak@radimsemerak.cz",
        job: "encode" 
    };

    try {
        const resProjects = await fetch("http://127.0.0.1:8080/api/projects/all");
        const resObfusc = await fetch("http://127.0.0.1:8080/api/obfusc", {
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body),
        });

        if (!resObfusc.ok) {
           return { err: await resObfusc.text() } 
        }

        const projects: ProjectResponse[] = await resProjects.json();
        const encoded: string = await resObfusc.json();

        return { projects, obfuscHandler: encoded };
    } catch (err) {
        return { err }
    }
};