type LoginRequest = {
    username: string,
    pw: string
}

type LoginResponse = {
    token?: {
        content: string,
        expires: number
    }
    err?: string
}

type TokenValidationReponse = {
    validated: boolean,
    err?: string
}