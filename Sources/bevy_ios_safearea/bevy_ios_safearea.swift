import UIKit

@_cdecl("swift_safearea_top")
public func safeareaTop(view: UnsafeRawPointer) -> Float32 {
    let view = Unmanaged<UIView>.fromOpaque(view).takeUnretainedValue()
    return Float32(view.safeAreaInsets.top);
}

@_cdecl("swift_safearea_bottom")
public func safeareaBottom(view: UnsafeRawPointer) -> Float32 {
    let view = Unmanaged<UIView>.fromOpaque(view).takeUnretainedValue()
    return Float32(view.safeAreaInsets.bottom);
}

