import type { Actions } from "./$types";

export const actions: Actions = {
    default: async ({ request }) => {
        const data = await request.formData();

        const res = await fetch("http://127.0.0.1:8080/api/projects/action", {
            method: "post",
            body: data 
        });

        if (!res.ok) {
            console.error(await res.text());
        } else {
            console.log("Project created!");
        }
    }
};