import { Word } from '@/lib/models';
import WordBank from '@/lib/WordBankClient';
import { LocalNotificationDescriptor, LocalNotifications, LocalNotificationSchema, ScheduleOn, ScheduleResult } from '@capacitor/local-notifications';
import storage from './storage';

export const initializeWordReminderNotifications = async (): Promise<void> => {
    
    // Delete existing local notifications
    const STORAGE_KEY = 'ln_word_reminders';
    await storage.get(STORAGE_KEY)
        .then((localNotifications: LocalNotificationDescriptor[]) => {
            return LocalNotifications.cancel({
                notifications: localNotifications,
            })
        })
        .catch(console.error);
    
    // Create new notifications
    const words: Word[] | undefined = await WordBank.random(4 * 7).catch((err) => {
        console.error(err);
        return undefined;
    });
    if(!words) {
        return;
    }

    // TODO: Schedule notifications
    const hours = [0, 10, 14, 20];
    const notifications = words
    .filter(w => w.translations.length > 0)
    .map((w, i) => ({
        id: i + 1,
        title: `Word reminder: ${w.value}`,
        body: `${w.value} = ${w.translations[0].value}`,
        schedule: {
            on: {
                weekday: i%7 + 1,
                hour: hours[i%hours.length],
            } as ScheduleOn
        },

    } as LocalNotificationSchema))

    return LocalNotifications.schedule({
        notifications: notifications,
    })
    .then((result: ScheduleResult) => {
        return storage.set(STORAGE_KEY, result.notifications);
    })
}