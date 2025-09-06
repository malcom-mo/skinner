import Cocoa

@discardableResult
func shell(_ args: [String]) -> Int32 {
    let task = Process()
    let appearance = NSApplication.shared.effectiveAppearance.name.rawValue.lowercased()
    let env = ProcessInfo.processInfo.environment

    // Switch theme based on system appearance
    if appearance.contains("dark") {
        task.launchPath = "/usr/bin/env"
        task.arguments = ["skinner", "activate", "dark"]
    } else if appearance.contains("light") {
        task.launchPath = "/usr/bin/env"
        task.arguments = ["skinner", "activate", "light"]
    } else {
        return 0
    }

    task.environment = env
    task.launch()
    task.waitUntilExit()
    return task.terminationStatus
}

let args = Array(CommandLine.arguments.suffix(from: 1))
shell(args)

DistributedNotificationCenter.default.addObserver(
    forName: Notification.Name("AppleInterfaceThemeChangedNotification"),
    object: nil,
    queue: nil
) { (notification) in
    DispatchQueue.main.asyncAfter(
        deadline: DispatchTime.now(),
        execute: {
            shell(args)
        })
}

NSWorkspace.shared.notificationCenter.addObserver(
    forName: NSWorkspace.didWakeNotification,
    object: nil,
    queue: nil
) { (notification) in
    shell(args)
}

NSApplication.shared.run()
