import { PageResult, Word } from "./models";

export default class WordBankClient {

    token: string;
    baseURL: string;

    constructor(token: string) {
        this.token = token;
        this.baseURL = "https://wordbank-api.herokuapp.com/"
    }

    listWords(): Promise<PageResult> {
        return this.doRequest<PageResult>(
            "GET",
            "api/v1/words",
        )
        .then((result) => {
            result.results = result.results.map(wObj => Word.fromObject(wObj));
            return result;
        });
    }


    private doRequest<T>(method: string, path: string, params: Record<string, any> = {}): Promise<T> {

        const query = {
            ...params,
            token: this.token
        };
        const queryString = Object.entries(query).map(([key, val]) => `${key}=${val}`).join("&");

        return fetch(this.baseURL.concat(path, '?', queryString), {
            method: method,
            headers: {
                'Authorization': `Bearer ${this.token}`,
            }
        })
        .then((res) => {
            return res.json();
        })
    }

}