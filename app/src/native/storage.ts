import { Storage } from '@ionic/storage';

const STORAGE = new Storage();

export const initializeStorage = (): Promise<Storage> => {
    return STORAGE.create();
}

export default STORAGE;