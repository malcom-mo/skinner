// Portions of this file are derived from
//   https://github.com/bouk/dark-mode-notify
//
// Copyright (c) 2021 Bouke van der Bijl
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Additional credit:
//   https://github.com/mnewt/dotemacs/blob/master/bin/dark-mode-notifier.swift
import Cocoa

@discardableResult
func switch_theme() -> Int32 {
    let task = Process()
    task.launchPath = "/usr/bin/env"

    let appearance = NSApplication.shared.effectiveAppearance.name.rawValue.lowercased()
    if appearance.contains("dark") {
        task.arguments = ["skinner", "activate", "dark"]
    } else {
        task.arguments = ["skinner", "activate", "light"]
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
    forName: NSWorkspace.screensDidWakeNotification,
    object: nil,
    queue: nil
) { (notification) in
    switch_theme()
}

NSApplication.shared.run()
