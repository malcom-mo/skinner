import Cocoa

@discardableResult
func switch_theme() -> Int32 {
    let task = Process()
    task.launchPath = "/opt/homebrew/bin/skinner"

    let appearance = NSApplication.shared.effectiveAppearance.name.rawValue.lowercased()
    if appearance.contains("dark") {
        task.arguments = ["activate", "dark"]
    } else if appearance.contains("light") {
        task.arguments = ["activate", "light"]
    } else {
        return 0
    }

    task.environment = ProcessInfo.processInfo.environment
    task.launch()
    task.waitUntilExit()
    return task.terminationStatus
}

switch_theme()

DistributedNotificationCenter.default.addObserver(
    forName: Notification.Name("AppleInterfaceThemeChangedNotification"),
    object: nil,
    queue: nil
) { (notification) in
    DispatchQueue.main.asyncAfter(
        deadline: DispatchTime.now(),
        execute: {
            switch_theme()
        })
}

NSWorkspace.shared.notificationCenter.addObserver(
    forName: NSWorkspace.didWakeNotification,
    object: nil,
    queue: nil
) { (notification) in
    switch_theme()
}

NSApplication.shared.run()
