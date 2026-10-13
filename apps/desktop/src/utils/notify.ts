import { toast } from "@quiro/ui";
import { commands } from "@/utils/tauri";

// Small windows like the launch toolbar clip in-window toasts, so feedback
// outside the editors goes through the OS notification center. A toast is the
// fallback when notifications are turned off in Settings or unavailable.
function show(message: string, isError: boolean) {
	commands
		.showNotification(message, null, isError)
		.catch(() => false)
		.then((shown) => {
			if (shown) return;
			if (isError) toast.error(message);
			else toast.success(message);
		});
}

export const notify = {
	success: (message: string) => show(message, false),
	error: (message: string) => show(message, true),
};
