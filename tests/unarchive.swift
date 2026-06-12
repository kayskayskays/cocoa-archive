import AppKit
import Foundation

func fail(_ message: String) -> Never {
  fputs(message + "\n", stderr)
  exit(1)
}

func test_color(_ color: NSColor) {
  print("decoded color:", color)

  guard
    color.redComponent == 1.0,
    color.greenComponent == 0.0,
    color.blueComponent == 0.0,
    color.alphaComponent == 1.0
  else {
    fail(
      "decoded NSColor, but color components were wrong: \(color.redComponent) \(color.greenComponent) \(color.blueComponent) \(color.alphaComponent)"
    )
  }
}

func test_font(_ font: NSFont) {
  print("decoded font:", font)

  guard font.fontName == "Helvetica" else {
    fail("decoded NSFont, but font name was wrong: \(font.fontName)")
  }

  guard font.pointSize == 12.0 else {
    fail("decoded NSFont, but point size was wrong: \(font.pointSize)")
  }
}

guard CommandLine.arguments.count == 3 else {
  fail("usage: swift unarchiver.swift <color|font> <base64>")
}

let expectedObject = CommandLine.arguments[1]
let base64 = CommandLine.arguments[2]

guard expectedObject == "color" || expectedObject == "font" else {
  fail("expected object must be 'color' or 'font'")
}

guard let data = Data(base64Encoded: base64) else {
  fail("invalid base64")
}

do {
  let object = try NSKeyedUnarchiver.unarchivedObject(
    ofClasses: [
      NSColor.self,
      NSFont.self,
      NSString.self,
      NSNumber.self,
      NSArray.self,
      NSDictionary.self,
      NSData.self,
    ],
    from: data
  )

  switch (expectedObject, object) {
  case ("color", let color as NSColor):
    test_color(color)

  case ("font", let font as NSFont):
    test_font(font)

  default:
    fail("decoded unexpected object: \(String(describing: object))")
  }

  exit(0)
} catch {
  fail("unarchive failed: \(error)")
}
