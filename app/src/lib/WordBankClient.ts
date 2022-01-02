import { Folder, FolderResult, PageResult, PaginationOptions, Word, WordQueryOptions } from "./models";

export default class WordBank {

    static token: string = process.env.VUE_APP_WORDBANK_API_TOKEN;
    static baseURL: string = process.env.VUE_APP_WORDBANK_API_URL // "https://wordbank-api.herokuapp.com/";

    static queryWords(query: WordQueryOptions = {}, pagination: PaginationOptions = {}): Promise<PageResult> {
        return this.doRequest(
            "GET",
            "api/v1/words",
            {
                ...query,
                ...pagination,
            }
        )
        .then((res) => {
            return res.json();
        })
        .then((result: PageResult) => {
            result.results = result.results.map(wObj => Word.fromObject(wObj));
            return result;
        });
    }

    static insertWord(word: Word, folderId?: string): Promise<Word> {
        return this.doRequest(
            "POST",
            "api/v1/words",
            undefined,
            {
                ...word.toObject(),
                translations: word.translations.map(it => it.value),
                folder: folderId,
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

    static getFolder(id: string): Promise<FolderResult> {
        return this.doRequest(
            "GET",
            "api/v1/folders/".concat(id),
        ).then((res) => res.json())
        .then((result: FolderResult) => {
            const data = result;
            data.data = Folder.fromObject(data.data);
            data.content.folders = data.content.folders.map(f => Folder.fromObject(f));
            data.content.words = data.content.words.map(w => Word.fromObject(w));
            return data;
        });
    }

    static insertFolder(folder: Folder): Promise<Folder> {
        return this.doRequest(
            "POST",
            "api/v1/folders",
            undefined,
            folder.toObject()
        )
        .then((res) => res.json())
        .then((f) => Folder.fromObject(f));
    }

    static deleteFolder(id: string): Promise<string> {
        return this.doRequest(
            "DELETE",
            "api/v1/folders/".concat(id)
        ).then(() => id);
    }

    static updateFolder(folder: Folder): Promise<Folder> {
        return this.doRequest(
            "PUT",
            "api/v1/folders/".concat(folder.id),
            undefined,
            folder.toObject(),
        ).then(() => {
            return folder;
        });
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
            .then((res) => {
                /* if(res.status >= 400) {
                    throw 
                } */
                return res;
            })
    }

}