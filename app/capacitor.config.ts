import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'com.andorr.word-bank',
  appName: 'WordBank',
  webDir: 'dist',
  bundledWebRuntime: false,
  plugins: {
    LocalNotifications: {
      // Android Local Notifications:
      // smallIcon: "ic_stat_icon_config_sample",
      // iconColor: "#488AFF",
      // sound: "beep.wav",
    }
  }
};


export default config;
