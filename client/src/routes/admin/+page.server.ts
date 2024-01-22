import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "../login/$types";

export const load: PageServerLoad = async ({ cookies }) => {
    const token = cookies.get("token");

    if (token) {
        const res = await fetch("http://127.0.0.1:8080/api/validate", {
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(token)
        });

        if (res.ok) {
            const tokenValidRes: TokenValidationReponse = await res.json();
            if (!tokenValidRes?.validated) {
                if (tokenValidRes?.err) {
                    handleInvalidToken(tokenValidRes.err);
                }
            }
        } else {
            handleInvalidToken(await res.text());
        }
    }
};

function handleInvalidToken(err: string) {
    console.error(err);
    redirect(301, "/login");
}