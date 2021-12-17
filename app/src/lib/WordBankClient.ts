import { PageResult, Word } from "./models";

export default class WordBank {

    static token: string = process.env.VUE_APP_WORDBANK_API_TOKEN;
    static baseURL: string = process.env.VUE_APP_WORDBANK_API_URL // "https://wordbank-api.herokuapp.com/";

    static listWords(): Promise<PageResult> {
        return this.doRequest<PageResult>(
            "GET",
            "api/v1/words",
        )
        .then((result) => {
            result.results = result.results.map(wObj => Word.fromObject(wObj));
            return result;
        });
    }

    static insertWord(word: Word): Promise<Word> {
        return this.doRequest<any>(
            "POST",
            "api/v1/words",
            undefined,
            word.toObject(),
        ).then((result) => {
            console.log("Result:", result)
            const newWord = Word.fromObject(result);
            
            const event = new CustomEvent('wb-word-insert', { detail: newWord });
            window.dispatchEvent(event);
            
            return newWord;
        })
    }
    
    static updateWord(word: Word): Promise<Word> {
        return this.doRequest<Word>(
            "PUT",
            "api/v1/words/".concat(word.id),
            undefined,
            word.toObject()
        ).then(() => {
            return word;
        });
    }


    private static doRequest<T>(method: string, path: string, params: Record<string, any> = {}, body: any = undefined): Promise<T> {
        console.log(path)
        const query = {
            ...params,
            token: this.token
        };
        const queryString = Object.entries(query).map(([key, val]) => `${key}=${val}`).join("&");

        const options: RequestInit = {
            method: method,
            headers: {
                'Authorization': `Bearer ${this.token}`,
                'Content-Type': 'application/json',
            },
        };
        if(body) {
            options.body = JSON.stringify(body);
        }

        return fetch(this.baseURL.concat(path, '?', queryString), options)
        .then((res) => {
            return res.json();
        })
    }

}