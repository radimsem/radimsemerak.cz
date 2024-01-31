import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "../login/$types";

export const load: PageServerLoad = async ({ cookies }) => {
    const token = cookies.get("token");

    if (token) {
        const body: TokenValidationRequest = handleTokenValidationRequest(token);
        const res = await fetch("http://127.0.0.1:8080/api/verify", {
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body)
        });

        if (!res.ok) handleInvalidToken(await res.text());
    } else {
        handleInvalidToken("There is no token at the moment!");
    }

    const res = await fetch("http://127.0.0.1:8080/api/projects/get");
    const projects: ProjectResponse[] = await res.json();

    return { projects };
};

function handleInvalidToken(err: string) {
    console.error(err);
    redirect(301, "/login");
}

function handleTokenValidationRequest(value: string): TokenValidationRequest {
    let params = value.split('|');

    return {
        id: parseInt(params.at(0) as string),
        client: params.at(1) as string
    }
}