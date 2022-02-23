import { alertController } from "@ionic/vue";

export const openErrorAlert = async (header: string, message: string, callback: (success: boolean) => void) => {
    const alert = await alertController.create({
        header: header,
        message: ``,
        buttons: [
          {
            text: "Yes",
            role: "yes",
          },
        ],
      });
      await alert.present();

      const { role } = await alert.onDidDismiss();
      callback(role === "yes")
}
