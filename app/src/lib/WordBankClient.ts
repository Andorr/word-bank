import { PageResult, Word } from "./models";

export default class WordBank {

    static token: string = process.env.VUE_APP_WORDBANK_API_TOKEN;
    static baseURL: string = process.env.VUE_APP_WORDBANK_API_URL // "https://wordbank-api.herokuapp.com/";

    static listWords(): Promise<PageResult> {
        return this.doRequest(
            "GET",
            "api/v1/words",
        )
        .then((res) => {
            return res.json();
        })
        .then((result: PageResult) => {
            result.results = result.results.map(wObj => Word.fromObject(wObj));
            return result;
        });
    }

    static insertWord(word: Word): Promise<Word> {
        return this.doRequest(
            "POST",
            "api/v1/words",
            undefined,
            {
                ...word.toObject(),
                translations: word.translations.map(it => it.value)
            },
        ).then((res) => {
            return res.json();
        }).then((result: object) => {
            const newWord = Word.fromObject(result);
            
            const event = new CustomEvent('wb-word-insert', { detail: newWord });
            window.dispatchEvent(event);
            
            return newWord;
        })
    }
    
    static updateWord(word: Word): Promise<Word> {
        return this.doRequest(
            "PUT",
            "api/v1/words/".concat(word.id),
            undefined,
            {
                ...word.toObject(),
                translations: word.translations.map(it => it.value)
            },
        ).then(() => {
            return word;
        });
    }
    
    static deleteWord(id: string): Promise<string> {
        return this.doRequest(
            "DELETE",
            "api/v1/words/".concat(id),
        ).then(() => id)
    }


    private static doRequest(method: string, path: string, params: Record<string, any> = {}, body: any = undefined): Promise<Response> {
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
    }

}