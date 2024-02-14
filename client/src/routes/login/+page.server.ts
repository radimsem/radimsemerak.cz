import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
    login: async ({ request, cookies }) => {
        const data = await request.formData();
        const body: LoginRequest = {
            username: data.get("username") as string,
            pw: data.get("pw") as string
        };

        const res = await fetch("http://127.0.0.1:8080/api/login", {
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body)
        });

        if (!res.ok) return { err: await res.text() };
        
        const token: LoginResponse = await res.json();
        const expires = new Date();

        expires.setTime(token.expires);
        cookies.set("token", `${token.id}|${token.content}`, { path: '/' });

        fetch("http://127.0.0.1:8080/api/expires");
        redirect(301, "/admin");
    }
} satisfies Actions;