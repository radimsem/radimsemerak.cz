type LoginRequest = {
    username: string,
    pw: string
}

type LoginResponse = {
    id: number,
    content: string,
    expires: number
}

type TokenValidationRequest = {
    id: number,
    client: string
}