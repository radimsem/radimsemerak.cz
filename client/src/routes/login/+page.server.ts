import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
    login: async ({ request, cookies }) => {
        const data = await request.formData();
        const loginReq: LoginRequest = {
            username: data.get("username") as string,
            pw: data.get("pw") as string
        };

        const res = await fetch("http://127.0.0.1:8080/api/login", {
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(loginReq)
        });

        if (!res.ok) {
            return { err: await res.text() };
        }
        
        const loginRes: LoginResponse = await res.json();
        if (loginRes.token) {
            let expires = new Date();
            expires.setTime(loginRes.token.expires);

            cookies.set("token", `id=${loginRes.token.id};content=${loginRes.token.content}`, { path: '/', expires });
            redirect(301, "/admin");
        } else {
            return { err: loginRes.err };
        }
    }
} satisfies Actions;