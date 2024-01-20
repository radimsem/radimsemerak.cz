import { redirect } from "@sveltejs/kit";

/** @type {import('./$types').Actions} */
export const actions = {
    login: async ({ request }) => {
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
        // const loginRes = await res.json();
        console.log(res);

        // if (loginRes.token) {
        //     throw redirect(301, "/admin");
        // } else {
        //     console.error(loginRes.err);
        // }
    }
};