import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions: Actions = {
    default: async ({ request }) => {
        const body = await request.formData();
        const res = await fetch("http://127.0.0.1:8080/api/projects/action", {
            method: "post",
            body
        });

        if (!res.ok) {
            return { err: await res.text() }
        }
        
        redirect(301, "/admin");
    }
};