#[inline]
pub unsafe fn AbortPath(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn AbortPath(hdc : HDC) -> windows_core::BOOL);
    unsafe { AbortPath(hdc) }
}
#[inline]
pub unsafe fn AddFontMemResourceEx(pfileview: *const core::ffi::c_void, cjsize: u32, pvresrved: Option<*const core::ffi::c_void>, pnumfonts: *const u32) -> super::super::Foundation::HANDLE {
    windows_link::link!("gdi32.dll" "system" fn AddFontMemResourceEx(pfileview : *const core::ffi::c_void, cjsize : u32, pvresrved : *const core::ffi::c_void, pnumfonts : *const u32) -> super::super::Foundation:: HANDLE);
    unsafe { AddFontMemResourceEx(pfileview, cjsize, pvresrved.unwrap_or(core::mem::zeroed()) as _, pnumfonts) }
}
#[inline]
pub unsafe fn AddFontResourceA<P0>(param0: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn AddFontResourceA(param0 : windows_core::PCSTR) -> i32);
    unsafe { AddFontResourceA(param0.param().abi()) }
}
#[inline]
pub unsafe fn AddFontResourceExA<P0>(name: P0, fl: FONT_RESOURCE_CHARACTERISTICS, res: Option<*const core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn AddFontResourceExA(name : windows_core::PCSTR, fl : FONT_RESOURCE_CHARACTERISTICS, res : *const core::ffi::c_void) -> i32);
    unsafe { AddFontResourceExA(name.param().abi(), fl, res.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AddFontResourceExW<P0>(name: P0, fl: FONT_RESOURCE_CHARACTERISTICS, res: Option<*const core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn AddFontResourceExW(name : windows_core::PCWSTR, fl : FONT_RESOURCE_CHARACTERISTICS, res : *const core::ffi::c_void) -> i32);
    unsafe { AddFontResourceExW(name.param().abi(), fl, res.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AddFontResourceW<P0>(param0: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn AddFontResourceW(param0 : windows_core::PCWSTR) -> i32);
    unsafe { AddFontResourceW(param0.param().abi()) }
}
#[inline]
pub unsafe fn AlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> windows_core::BOOL {
    windows_link::link!("msimg32.dll" "system" fn AlphaBlend(hdcdest : HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, ftn : BLENDFUNCTION) -> windows_core::BOOL);
    unsafe { AlphaBlend(hdcdest, xorigindest, yorigindest, wdest, hdest, hdcsrc, xoriginsrc, yoriginsrc, wsrc, hsrc, core::mem::transmute(ftn)) }
}
#[inline]
pub unsafe fn AngleArc(hdc: HDC, x: i32, y: i32, r: u32, startangle: f32, sweepangle: f32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn AngleArc(hdc : HDC, x : i32, y : i32, r : u32, startangle : f32, sweepangle : f32) -> windows_core::BOOL);
    unsafe { AngleArc(hdc, x, y, r, startangle, sweepangle) }
}
#[inline]
pub unsafe fn AnimatePalette(hpal: HPALETTE, istartindex: u32, ppe: &[PALETTEENTRY]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn AnimatePalette(hpal : HPALETTE, istartindex : u32, centries : u32, ppe : *const PALETTEENTRY) -> windows_core::BOOL);
    unsafe { AnimatePalette(hpal, istartindex, ppe.len().try_into().unwrap(), core::mem::transmute(ppe.as_ptr())) }
}
#[inline]
pub unsafe fn Arc(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn Arc(hdc : HDC, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> windows_core::BOOL);
    unsafe { Arc(hdc, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[inline]
pub unsafe fn ArcTo(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn ArcTo(hdc : HDC, left : i32, top : i32, right : i32, bottom : i32, xr1 : i32, yr1 : i32, xr2 : i32, yr2 : i32) -> windows_core::BOOL);
    unsafe { ArcTo(hdc, left, top, right, bottom, xr1, yr1, xr2, yr2) }
}
#[inline]
pub unsafe fn BeginPaint(hwnd: super::super::Foundation::HWND, lppaint: *mut PAINTSTRUCT) -> HDC {
    windows_link::link!("user32.dll" "system" fn BeginPaint(hwnd : super::super::Foundation:: HWND, lppaint : *mut PAINTSTRUCT) -> HDC);
    unsafe { BeginPaint(hwnd, lppaint as _) }
}
#[inline]
pub unsafe fn BeginPath(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn BeginPath(hdc : HDC) -> windows_core::BOOL);
    unsafe { BeginPath(hdc) }
}
#[inline]
pub unsafe fn BitBlt(hdc: HDC, x: i32, y: i32, cx: i32, cy: i32, hdcsrc: Option<HDC>, x1: i32, y1: i32, rop: ROP_CODE) -> windows_core::Result<()> {
    windows_link::link!("gdi32.dll" "system" fn BitBlt(hdc : HDC, x : i32, y : i32, cx : i32, cy : i32, hdcsrc : HDC, x1 : i32, y1 : i32, rop : ROP_CODE) -> windows_core::BOOL);
    unsafe { BitBlt(hdc, x, y, cx, cy, hdcsrc.unwrap_or(core::mem::zeroed()) as _, x1, y1, rop).ok() }
}
#[inline]
pub unsafe fn CancelDC(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn CancelDC(hdc : HDC) -> windows_core::BOOL);
    unsafe { CancelDC(hdc) }
}
#[inline]
pub unsafe fn ChangeDisplaySettingsA(lpdevmode: Option<*const DEVMODEA>, dwflags: CDS_TYPE) -> DISP_CHANGE {
    windows_link::link!("user32.dll" "system" fn ChangeDisplaySettingsA(lpdevmode : *const DEVMODEA, dwflags : CDS_TYPE) -> DISP_CHANGE);
    unsafe { ChangeDisplaySettingsA(lpdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn ChangeDisplaySettingsExA<P0>(lpszdevicename: P0, lpdevmode: Option<*const DEVMODEA>, hwnd: Option<super::super::Foundation::HWND>, dwflags: CDS_TYPE, lparam: Option<*const core::ffi::c_void>) -> DISP_CHANGE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("user32.dll" "system" fn ChangeDisplaySettingsExA(lpszdevicename : windows_core::PCSTR, lpdevmode : *const DEVMODEA, hwnd : super::super::Foundation:: HWND, dwflags : CDS_TYPE, lparam : *const core::ffi::c_void) -> DISP_CHANGE);
    unsafe { ChangeDisplaySettingsExA(lpszdevicename.param().abi(), lpdevmode.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, dwflags, lparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ChangeDisplaySettingsExW<P0>(lpszdevicename: P0, lpdevmode: Option<*const DEVMODEW>, hwnd: Option<super::super::Foundation::HWND>, dwflags: CDS_TYPE, lparam: Option<*const core::ffi::c_void>) -> DISP_CHANGE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("user32.dll" "system" fn ChangeDisplaySettingsExW(lpszdevicename : windows_core::PCWSTR, lpdevmode : *const DEVMODEW, hwnd : super::super::Foundation:: HWND, dwflags : CDS_TYPE, lparam : *const core::ffi::c_void) -> DISP_CHANGE);
    unsafe { ChangeDisplaySettingsExW(lpszdevicename.param().abi(), lpdevmode.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, dwflags, lparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ChangeDisplaySettingsW(lpdevmode: Option<*const DEVMODEW>, dwflags: CDS_TYPE) -> DISP_CHANGE {
    windows_link::link!("user32.dll" "system" fn ChangeDisplaySettingsW(lpdevmode : *const DEVMODEW, dwflags : CDS_TYPE) -> DISP_CHANGE);
    unsafe { ChangeDisplaySettingsW(lpdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn Chord(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn Chord(hdc : HDC, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> windows_core::BOOL);
    unsafe { Chord(hdc, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[inline]
pub unsafe fn ClientToScreen(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn ClientToScreen(hwnd : super::super::Foundation:: HWND, lppoint : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { ClientToScreen(hwnd, lppoint as _) }
}
#[inline]
pub unsafe fn CloseEnhMetaFile(hdc: HDC) -> HENHMETAFILE {
    windows_link::link!("gdi32.dll" "system" fn CloseEnhMetaFile(hdc : HDC) -> HENHMETAFILE);
    unsafe { CloseEnhMetaFile(hdc) }
}
#[inline]
pub unsafe fn CloseFigure(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn CloseFigure(hdc : HDC) -> windows_core::BOOL);
    unsafe { CloseFigure(hdc) }
}
#[inline]
pub unsafe fn CloseMetaFile(hdc: HDC) -> HMETAFILE {
    windows_link::link!("gdi32.dll" "system" fn CloseMetaFile(hdc : HDC) -> HMETAFILE);
    unsafe { CloseMetaFile(hdc) }
}
#[inline]
pub unsafe fn CombineRgn(hrgndst: Option<HRGN>, hrgnsrc1: Option<HRGN>, hrgnsrc2: Option<HRGN>, imode: RGN_COMBINE_MODE) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn CombineRgn(hrgndst : HRGN, hrgnsrc1 : HRGN, hrgnsrc2 : HRGN, imode : RGN_COMBINE_MODE) -> GDI_REGION_TYPE);
    unsafe { CombineRgn(hrgndst.unwrap_or(core::mem::zeroed()) as _, hrgnsrc1.unwrap_or(core::mem::zeroed()) as _, hrgnsrc2.unwrap_or(core::mem::zeroed()) as _, imode) }
}
#[inline]
pub unsafe fn CombineTransform(lpxfout: *mut XFORM, lpxf1: *const XFORM, lpxf2: *const XFORM) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn CombineTransform(lpxfout : *mut XFORM, lpxf1 : *const XFORM, lpxf2 : *const XFORM) -> windows_core::BOOL);
    unsafe { CombineTransform(lpxfout as _, lpxf1, lpxf2) }
}
#[inline]
pub unsafe fn CopyEnhMetaFileA<P1>(henh: HENHMETAFILE, lpfilename: P1) -> HENHMETAFILE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CopyEnhMetaFileA(henh : HENHMETAFILE, lpfilename : windows_core::PCSTR) -> HENHMETAFILE);
    unsafe { CopyEnhMetaFileA(henh, lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn CopyEnhMetaFileW<P1>(henh: HENHMETAFILE, lpfilename: P1) -> HENHMETAFILE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CopyEnhMetaFileW(henh : HENHMETAFILE, lpfilename : windows_core::PCWSTR) -> HENHMETAFILE);
    unsafe { CopyEnhMetaFileW(henh, lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn CopyMetaFileA<P1>(param0: HMETAFILE, param1: P1) -> HMETAFILE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CopyMetaFileA(param0 : HMETAFILE, param1 : windows_core::PCSTR) -> HMETAFILE);
    unsafe { CopyMetaFileA(param0, param1.param().abi()) }
}
#[inline]
pub unsafe fn CopyMetaFileW<P1>(param0: HMETAFILE, param1: P1) -> HMETAFILE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CopyMetaFileW(param0 : HMETAFILE, param1 : windows_core::PCWSTR) -> HMETAFILE);
    unsafe { CopyMetaFileW(param0, param1.param().abi()) }
}
#[inline]
pub unsafe fn CopyRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn CopyRect(lprcdst : *mut super::super::Foundation:: RECT, lprcsrc : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { CopyRect(lprcdst as _, lprcsrc) }
}
#[inline]
pub unsafe fn CreateBitmap(nwidth: i32, nheight: i32, nplanes: u32, nbitcount: u32, lpbits: Option<*const core::ffi::c_void>) -> HBITMAP {
    windows_link::link!("gdi32.dll" "system" fn CreateBitmap(nwidth : i32, nheight : i32, nplanes : u32, nbitcount : u32, lpbits : *const core::ffi::c_void) -> HBITMAP);
    unsafe { CreateBitmap(nwidth, nheight, nplanes, nbitcount, lpbits.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateBitmapIndirect(pbm: *const BITMAP) -> HBITMAP {
    windows_link::link!("gdi32.dll" "system" fn CreateBitmapIndirect(pbm : *const BITMAP) -> HBITMAP);
    unsafe { CreateBitmapIndirect(pbm) }
}
#[inline]
pub unsafe fn CreateBrushIndirect(plbrush: *const LOGBRUSH) -> HBRUSH {
    windows_link::link!("gdi32.dll" "system" fn CreateBrushIndirect(plbrush : *const LOGBRUSH) -> HBRUSH);
    unsafe { CreateBrushIndirect(plbrush) }
}
#[inline]
pub unsafe fn CreateCompatibleBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP {
    windows_link::link!("gdi32.dll" "system" fn CreateCompatibleBitmap(hdc : HDC, cx : i32, cy : i32) -> HBITMAP);
    unsafe { CreateCompatibleBitmap(hdc, cx, cy) }
}
#[inline]
pub unsafe fn CreateCompatibleDC(hdc: Option<HDC>) -> HDC {
    windows_link::link!("gdi32.dll" "system" fn CreateCompatibleDC(hdc : HDC) -> HDC);
    unsafe { CreateCompatibleDC(hdc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateDCA<P0, P1, P2>(pwszdriver: P0, pwszdevice: P1, pszport: P2, pdm: Option<*const DEVMODEA>) -> HDC
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateDCA(pwszdriver : windows_core::PCSTR, pwszdevice : windows_core::PCSTR, pszport : windows_core::PCSTR, pdm : *const DEVMODEA) -> HDC);
    unsafe { CreateDCA(pwszdriver.param().abi(), pwszdevice.param().abi(), pszport.param().abi(), pdm.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateDCW<P0, P1, P2>(pwszdriver: P0, pwszdevice: P1, pszport: P2, pdm: Option<*const DEVMODEW>) -> HDC
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateDCW(pwszdriver : windows_core::PCWSTR, pwszdevice : windows_core::PCWSTR, pszport : windows_core::PCWSTR, pdm : *const DEVMODEW) -> HDC);
    unsafe { CreateDCW(pwszdriver.param().abi(), pwszdevice.param().abi(), pszport.param().abi(), pdm.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateDIBPatternBrush(h: super::super::Foundation::HGLOBAL, iusage: DIB_USAGE) -> HBRUSH {
    windows_link::link!("gdi32.dll" "system" fn CreateDIBPatternBrush(h : super::super::Foundation:: HGLOBAL, iusage : DIB_USAGE) -> HBRUSH);
    unsafe { CreateDIBPatternBrush(h, iusage) }
}
#[inline]
pub unsafe fn CreateDIBPatternBrushPt(lppackeddib: *const core::ffi::c_void, iusage: DIB_USAGE) -> HBRUSH {
    windows_link::link!("gdi32.dll" "system" fn CreateDIBPatternBrushPt(lppackeddib : *const core::ffi::c_void, iusage : DIB_USAGE) -> HBRUSH);
    unsafe { CreateDIBPatternBrushPt(lppackeddib, iusage) }
}
#[inline]
pub unsafe fn CreateDIBSection(hdc: Option<HDC>, pbmi: *const BITMAPINFO, usage: DIB_USAGE, ppvbits: *mut *mut core::ffi::c_void, hsection: Option<super::super::Foundation::HANDLE>, offset: u32) -> windows_core::Result<HBITMAP> {
    windows_link::link!("gdi32.dll" "system" fn CreateDIBSection(hdc : HDC, pbmi : *const BITMAPINFO, usage : DIB_USAGE, ppvbits : *mut *mut core::ffi::c_void, hsection : super::super::Foundation:: HANDLE, offset : u32) -> HBITMAP);
    let result__ = unsafe { CreateDIBSection(hdc.unwrap_or(core::mem::zeroed()) as _, pbmi, usage, ppvbits as _, hsection.unwrap_or(core::mem::zeroed()) as _, offset) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateDIBitmap(hdc: HDC, pbmih: Option<*const BITMAPINFOHEADER>, flinit: u32, pjbits: Option<*const core::ffi::c_void>, pbmi: Option<*const BITMAPINFO>, iusage: DIB_USAGE) -> HBITMAP {
    windows_link::link!("gdi32.dll" "system" fn CreateDIBitmap(hdc : HDC, pbmih : *const BITMAPINFOHEADER, flinit : u32, pjbits : *const core::ffi::c_void, pbmi : *const BITMAPINFO, iusage : DIB_USAGE) -> HBITMAP);
    unsafe { CreateDIBitmap(hdc, pbmih.unwrap_or(core::mem::zeroed()) as _, flinit, pjbits.unwrap_or(core::mem::zeroed()) as _, pbmi.unwrap_or(core::mem::zeroed()) as _, iusage) }
}
#[inline]
pub unsafe fn CreateDiscardableBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP {
    windows_link::link!("gdi32.dll" "system" fn CreateDiscardableBitmap(hdc : HDC, cx : i32, cy : i32) -> HBITMAP);
    unsafe { CreateDiscardableBitmap(hdc, cx, cy) }
}
#[inline]
pub unsafe fn CreateEllipticRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn CreateEllipticRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> HRGN);
    unsafe { CreateEllipticRgn(x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn CreateEllipticRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn CreateEllipticRgnIndirect(lprect : *const super::super::Foundation:: RECT) -> HRGN);
    unsafe { CreateEllipticRgnIndirect(lprect) }
}
#[inline]
pub unsafe fn CreateEnhMetaFileA<P1, P3>(hdc: Option<HDC>, lpfilename: P1, lprc: Option<*const super::super::Foundation::RECT>, lpdesc: P3) -> HDC
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateEnhMetaFileA(hdc : HDC, lpfilename : windows_core::PCSTR, lprc : *const super::super::Foundation:: RECT, lpdesc : windows_core::PCSTR) -> HDC);
    unsafe { CreateEnhMetaFileA(hdc.unwrap_or(core::mem::zeroed()) as _, lpfilename.param().abi(), lprc.unwrap_or(core::mem::zeroed()) as _, lpdesc.param().abi()) }
}
#[inline]
pub unsafe fn CreateEnhMetaFileW<P1, P3>(hdc: Option<HDC>, lpfilename: P1, lprc: Option<*const super::super::Foundation::RECT>, lpdesc: P3) -> HDC
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateEnhMetaFileW(hdc : HDC, lpfilename : windows_core::PCWSTR, lprc : *const super::super::Foundation:: RECT, lpdesc : windows_core::PCWSTR) -> HDC);
    unsafe { CreateEnhMetaFileW(hdc.unwrap_or(core::mem::zeroed()) as _, lpfilename.param().abi(), lprc.unwrap_or(core::mem::zeroed()) as _, lpdesc.param().abi()) }
}
#[inline]
pub unsafe fn CreateFontA<P13>(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: FONT_CHARSET, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: u32, pszfacename: P13) -> HFONT
where
    P13: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateFontA(cheight : i32, cwidth : i32, cescapement : i32, corientation : i32, cweight : i32, bitalic : u32, bunderline : u32, bstrikeout : u32, icharset : u32, ioutprecision : u32, iclipprecision : u32, iquality : u32, ipitchandfamily : u32, pszfacename : windows_core::PCSTR) -> HFONT);
    unsafe { CreateFontA(cheight, cwidth, cescapement, corientation, cweight, bitalic, bunderline, bstrikeout, icharset.0 as _, ioutprecision.0 as _, iclipprecision.0 as _, iquality.0 as _, ipitchandfamily, pszfacename.param().abi()) }
}
#[inline]
pub unsafe fn CreateFontIndirectA(lplf: *const LOGFONTA) -> HFONT {
    windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectA(lplf : *const LOGFONTA) -> HFONT);
    unsafe { CreateFontIndirectA(lplf) }
}
#[inline]
pub unsafe fn CreateFontIndirectExA(param0: *const ENUMLOGFONTEXDVA) -> HFONT {
    windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectExA(param0 : *const ENUMLOGFONTEXDVA) -> HFONT);
    unsafe { CreateFontIndirectExA(param0) }
}
#[inline]
pub unsafe fn CreateFontIndirectExW(param0: *const ENUMLOGFONTEXDVW) -> HFONT {
    windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectExW(param0 : *const ENUMLOGFONTEXDVW) -> HFONT);
    unsafe { CreateFontIndirectExW(param0) }
}
#[inline]
pub unsafe fn CreateFontIndirectW(lplf: *const LOGFONTW) -> HFONT {
    windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectW(lplf : *const LOGFONTW) -> HFONT);
    unsafe { CreateFontIndirectW(lplf) }
}
#[inline]
pub unsafe fn CreateFontPackage(puchsrcbuffer: *const u8, ulsrcbuffersize: u32, ppuchfontpackagebuffer: *mut *mut u8, pulfontpackagebuffersize: *mut u32, pulbyteswritten: *mut u32, usflag: u16, usttcindex: u16, ussubsetformat: u16, ussubsetlanguage: u16, ussubsetplatform: CREATE_FONT_PACKAGE_SUBSET_PLATFORM, ussubsetencoding: CREATE_FONT_PACKAGE_SUBSET_ENCODING, pussubsetkeeplist: *const u16, ussubsetlistcount: u16, lpfnallocate: CFP_ALLOCPROC, lpfnreallocate: CFP_REALLOCPROC, lpfnfree: CFP_FREEPROC, lpvreserved: *mut core::ffi::c_void) -> u32 {
    windows_link::link!("fontsub.dll" "C" fn CreateFontPackage(puchsrcbuffer : *const u8, ulsrcbuffersize : u32, ppuchfontpackagebuffer : *mut *mut u8, pulfontpackagebuffersize : *mut u32, pulbyteswritten : *mut u32, usflag : u16, usttcindex : u16, ussubsetformat : u16, ussubsetlanguage : u16, ussubsetplatform : CREATE_FONT_PACKAGE_SUBSET_PLATFORM, ussubsetencoding : CREATE_FONT_PACKAGE_SUBSET_ENCODING, pussubsetkeeplist : *const u16, ussubsetlistcount : u16, lpfnallocate : CFP_ALLOCPROC, lpfnreallocate : CFP_REALLOCPROC, lpfnfree : CFP_FREEPROC, lpvreserved : *mut core::ffi::c_void) -> u32);
    unsafe { CreateFontPackage(puchsrcbuffer, ulsrcbuffersize, ppuchfontpackagebuffer as _, pulfontpackagebuffersize as _, pulbyteswritten as _, usflag, usttcindex, ussubsetformat, ussubsetlanguage, ussubsetplatform, ussubsetencoding, pussubsetkeeplist, ussubsetlistcount, lpfnallocate, lpfnreallocate, lpfnfree, lpvreserved as _) }
}
#[inline]
pub unsafe fn CreateFontW<P13>(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: FONT_CHARSET, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: u32, pszfacename: P13) -> HFONT
where
    P13: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateFontW(cheight : i32, cwidth : i32, cescapement : i32, corientation : i32, cweight : i32, bitalic : u32, bunderline : u32, bstrikeout : u32, icharset : u32, ioutprecision : u32, iclipprecision : u32, iquality : u32, ipitchandfamily : u32, pszfacename : windows_core::PCWSTR) -> HFONT);
    unsafe { CreateFontW(cheight, cwidth, cescapement, corientation, cweight, bitalic, bunderline, bstrikeout, icharset.0 as _, ioutprecision.0 as _, iclipprecision.0 as _, iquality.0 as _, ipitchandfamily, pszfacename.param().abi()) }
}
#[inline]
pub unsafe fn CreateHalftonePalette(hdc: Option<HDC>) -> HPALETTE {
    windows_link::link!("gdi32.dll" "system" fn CreateHalftonePalette(hdc : HDC) -> HPALETTE);
    unsafe { CreateHalftonePalette(hdc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateHatchBrush(ihatch: HATCH_BRUSH_STYLE, color: super::super::Foundation::COLORREF) -> HBRUSH {
    windows_link::link!("gdi32.dll" "system" fn CreateHatchBrush(ihatch : HATCH_BRUSH_STYLE, color : super::super::Foundation:: COLORREF) -> HBRUSH);
    unsafe { CreateHatchBrush(ihatch, color) }
}
#[inline]
pub unsafe fn CreateICA<P0, P1, P2>(pszdriver: P0, pszdevice: P1, pszport: P2, pdm: Option<*const DEVMODEA>) -> HDC
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateICA(pszdriver : windows_core::PCSTR, pszdevice : windows_core::PCSTR, pszport : windows_core::PCSTR, pdm : *const DEVMODEA) -> HDC);
    unsafe { CreateICA(pszdriver.param().abi(), pszdevice.param().abi(), pszport.param().abi(), pdm.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateICW<P0, P1, P2>(pszdriver: P0, pszdevice: P1, pszport: P2, pdm: Option<*const DEVMODEW>) -> HDC
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateICW(pszdriver : windows_core::PCWSTR, pszdevice : windows_core::PCWSTR, pszport : windows_core::PCWSTR, pdm : *const DEVMODEW) -> HDC);
    unsafe { CreateICW(pszdriver.param().abi(), pszdevice.param().abi(), pszport.param().abi(), pdm.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateMetaFileA<P0>(pszfile: P0) -> HDC
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateMetaFileA(pszfile : windows_core::PCSTR) -> HDC);
    unsafe { CreateMetaFileA(pszfile.param().abi()) }
}
#[inline]
pub unsafe fn CreateMetaFileW<P0>(pszfile: P0) -> HDC
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateMetaFileW(pszfile : windows_core::PCWSTR) -> HDC);
    unsafe { CreateMetaFileW(pszfile.param().abi()) }
}
#[inline]
pub unsafe fn CreatePalette(plpal: *const LOGPALETTE) -> HPALETTE {
    windows_link::link!("gdi32.dll" "system" fn CreatePalette(plpal : *const LOGPALETTE) -> HPALETTE);
    unsafe { CreatePalette(plpal) }
}
#[inline]
pub unsafe fn CreatePatternBrush(hbm: HBITMAP) -> HBRUSH {
    windows_link::link!("gdi32.dll" "system" fn CreatePatternBrush(hbm : HBITMAP) -> HBRUSH);
    unsafe { CreatePatternBrush(hbm) }
}
#[inline]
pub unsafe fn CreatePen(istyle: PEN_STYLE, cwidth: i32, color: super::super::Foundation::COLORREF) -> HPEN {
    windows_link::link!("gdi32.dll" "system" fn CreatePen(istyle : PEN_STYLE, cwidth : i32, color : super::super::Foundation:: COLORREF) -> HPEN);
    unsafe { CreatePen(istyle, cwidth, color) }
}
#[inline]
pub unsafe fn CreatePenIndirect(plpen: *const LOGPEN) -> HPEN {
    windows_link::link!("gdi32.dll" "system" fn CreatePenIndirect(plpen : *const LOGPEN) -> HPEN);
    unsafe { CreatePenIndirect(plpen) }
}
#[inline]
pub unsafe fn CreatePolyPolygonRgn(pptl: *const super::super::Foundation::POINT, pc: &[i32], imode: CREATE_POLYGON_RGN_MODE) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn CreatePolyPolygonRgn(pptl : *const super::super::Foundation:: POINT, pc : *const i32, cpoly : i32, imode : CREATE_POLYGON_RGN_MODE) -> HRGN);
    unsafe { CreatePolyPolygonRgn(pptl, core::mem::transmute(pc.as_ptr()), pc.len().try_into().unwrap(), imode) }
}
#[inline]
pub unsafe fn CreatePolygonRgn(pptl: &[super::super::Foundation::POINT], imode: CREATE_POLYGON_RGN_MODE) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn CreatePolygonRgn(pptl : *const super::super::Foundation:: POINT, cpoint : i32, imode : CREATE_POLYGON_RGN_MODE) -> HRGN);
    unsafe { CreatePolygonRgn(core::mem::transmute(pptl.as_ptr()), pptl.len().try_into().unwrap(), imode) }
}
#[inline]
pub unsafe fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn CreateRectRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> HRGN);
    unsafe { CreateRectRgn(x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn CreateRectRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn CreateRectRgnIndirect(lprect : *const super::super::Foundation:: RECT) -> HRGN);
    unsafe { CreateRectRgnIndirect(lprect) }
}
#[inline]
pub unsafe fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn CreateRoundRectRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32, w : i32, h : i32) -> HRGN);
    unsafe { CreateRoundRectRgn(x1, y1, x2, y2, w, h) }
}
#[inline]
pub unsafe fn CreateScalableFontResourceA<P1, P2, P3>(fdwhidden: u32, lpszfont: P1, lpszfile: P2, lpszpath: P3) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateScalableFontResourceA(fdwhidden : u32, lpszfont : windows_core::PCSTR, lpszfile : windows_core::PCSTR, lpszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CreateScalableFontResourceA(fdwhidden, lpszfont.param().abi(), lpszfile.param().abi(), lpszpath.param().abi()).ok() }
}
#[inline]
pub unsafe fn CreateScalableFontResourceW<P1, P2, P3>(fdwhidden: u32, lpszfont: P1, lpszfile: P2, lpszpath: P3) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn CreateScalableFontResourceW(fdwhidden : u32, lpszfont : windows_core::PCWSTR, lpszfile : windows_core::PCWSTR, lpszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CreateScalableFontResourceW(fdwhidden, lpszfont.param().abi(), lpszfile.param().abi(), lpszpath.param().abi()).ok() }
}
#[inline]
pub unsafe fn CreateSolidBrush(color: super::super::Foundation::COLORREF) -> HBRUSH {
    windows_link::link!("gdi32.dll" "system" fn CreateSolidBrush(color : super::super::Foundation:: COLORREF) -> HBRUSH);
    unsafe { CreateSolidBrush(color) }
}
#[inline]
pub unsafe fn DPtoLP(hdc: HDC, lppt: &mut [super::super::Foundation::POINT]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn DPtoLP(hdc : HDC, lppt : *mut super::super::Foundation:: POINT, c : i32) -> windows_core::BOOL);
    unsafe { DPtoLP(hdc, core::mem::transmute(lppt.as_ptr()), lppt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn DeleteDC(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn DeleteDC(hdc : HDC) -> windows_core::BOOL);
    unsafe { DeleteDC(hdc) }
}
#[inline]
pub unsafe fn DeleteEnhMetaFile(hmf: Option<HENHMETAFILE>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn DeleteEnhMetaFile(hmf : HENHMETAFILE) -> windows_core::BOOL);
    unsafe { DeleteEnhMetaFile(hmf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteMetaFile(hmf: HMETAFILE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn DeleteMetaFile(hmf : HMETAFILE) -> windows_core::BOOL);
    unsafe { DeleteMetaFile(hmf) }
}
#[inline]
pub unsafe fn DeleteObject(ho: HGDIOBJ) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : HGDIOBJ) -> windows_core::BOOL);
    unsafe { DeleteObject(ho) }
}
#[inline]
pub unsafe fn DrawAnimatedRects(hwnd: Option<super::super::Foundation::HWND>, idani: i32, lprcfrom: *const super::super::Foundation::RECT, lprcto: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn DrawAnimatedRects(hwnd : super::super::Foundation:: HWND, idani : i32, lprcfrom : *const super::super::Foundation:: RECT, lprcto : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { DrawAnimatedRects(hwnd.unwrap_or(core::mem::zeroed()) as _, idani, lprcfrom, lprcto) }
}
#[inline]
pub unsafe fn DrawCaption(hwnd: super::super::Foundation::HWND, hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: DRAW_CAPTION_FLAGS) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn DrawCaption(hwnd : super::super::Foundation:: HWND, hdc : HDC, lprect : *const super::super::Foundation:: RECT, flags : DRAW_CAPTION_FLAGS) -> windows_core::BOOL);
    unsafe { DrawCaption(hwnd, hdc, lprect, flags) }
}
#[inline]
pub unsafe fn DrawEdge(hdc: HDC, qrc: *mut super::super::Foundation::RECT, edge: DRAWEDGE_FLAGS, grfflags: DRAW_EDGE_FLAGS) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn DrawEdge(hdc : HDC, qrc : *mut super::super::Foundation:: RECT, edge : DRAWEDGE_FLAGS, grfflags : DRAW_EDGE_FLAGS) -> windows_core::BOOL);
    unsafe { DrawEdge(hdc, qrc as _, edge, grfflags) }
}
#[inline]
pub unsafe fn DrawEscape(hdc: HDC, iescape: i32, lpin: Option<&[u8]>) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn DrawEscape(hdc : HDC, iescape : i32, cjin : i32, lpin : windows_core::PCSTR) -> i32);
    unsafe { DrawEscape(hdc, iescape, lpin.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpin.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn DrawFocusRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn DrawFocusRect(hdc : HDC, lprc : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { DrawFocusRect(hdc, lprc) }
}
#[inline]
pub unsafe fn DrawFrameControl(param0: HDC, param1: *mut super::super::Foundation::RECT, param2: DFC_TYPE, param3: DFCS_STATE) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn DrawFrameControl(param0 : HDC, param1 : *mut super::super::Foundation:: RECT, param2 : DFC_TYPE, param3 : DFCS_STATE) -> windows_core::BOOL);
    unsafe { DrawFrameControl(param0, param1 as _, param2, param3) }
}
#[inline]
pub unsafe fn DrawStateA(hdc: HDC, hbrfore: Option<HBRUSH>, qfncallback: DRAWSTATEPROC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn DrawStateA(hdc : HDC, hbrfore : HBRUSH, qfncallback : DRAWSTATEPROC, ldata : super::super::Foundation:: LPARAM, wdata : super::super::Foundation:: WPARAM, x : i32, y : i32, cx : i32, cy : i32, uflags : DRAWSTATE_FLAGS) -> windows_core::BOOL);
    unsafe { DrawStateA(hdc, hbrfore.unwrap_or(core::mem::zeroed()) as _, qfncallback, ldata, wdata, x, y, cx, cy, uflags) }
}
#[inline]
pub unsafe fn DrawStateW(hdc: HDC, hbrfore: Option<HBRUSH>, qfncallback: DRAWSTATEPROC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn DrawStateW(hdc : HDC, hbrfore : HBRUSH, qfncallback : DRAWSTATEPROC, ldata : super::super::Foundation:: LPARAM, wdata : super::super::Foundation:: WPARAM, x : i32, y : i32, cx : i32, cy : i32, uflags : DRAWSTATE_FLAGS) -> windows_core::BOOL);
    unsafe { DrawStateW(hdc, hbrfore.unwrap_or(core::mem::zeroed()) as _, qfncallback, ldata, wdata, x, y, cx, cy, uflags) }
}
#[inline]
pub unsafe fn DrawTextA(hdc: HDC, lpchtext: &mut [u8], lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32 {
    windows_link::link!("user32.dll" "system" fn DrawTextA(hdc : HDC, lpchtext : windows_core::PCSTR, cchtext : i32, lprc : *mut super::super::Foundation:: RECT, format : DRAW_TEXT_FORMAT) -> i32);
    unsafe { DrawTextA(hdc, core::mem::transmute(lpchtext.as_ptr()), lpchtext.len().try_into().unwrap(), lprc as _, format) }
}
#[inline]
pub unsafe fn DrawTextExA(hdc: HDC, lpchtext: &mut [u8], lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: Option<*const DRAWTEXTPARAMS>) -> i32 {
    windows_link::link!("user32.dll" "system" fn DrawTextExA(hdc : HDC, lpchtext : windows_core::PSTR, cchtext : i32, lprc : *mut super::super::Foundation:: RECT, format : DRAW_TEXT_FORMAT, lpdtp : *const DRAWTEXTPARAMS) -> i32);
    unsafe { DrawTextExA(hdc, core::mem::transmute(lpchtext.as_ptr()), lpchtext.len().try_into().unwrap(), lprc as _, format, lpdtp.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DrawTextExW(hdc: HDC, lpchtext: &mut [u16], lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: Option<*const DRAWTEXTPARAMS>) -> i32 {
    windows_link::link!("user32.dll" "system" fn DrawTextExW(hdc : HDC, lpchtext : windows_core::PWSTR, cchtext : i32, lprc : *mut super::super::Foundation:: RECT, format : DRAW_TEXT_FORMAT, lpdtp : *const DRAWTEXTPARAMS) -> i32);
    unsafe { DrawTextExW(hdc, core::mem::transmute(lpchtext.as_ptr()), lpchtext.len().try_into().unwrap(), lprc as _, format, lpdtp.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DrawTextW(hdc: HDC, lpchtext: &mut [u16], lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32 {
    windows_link::link!("user32.dll" "system" fn DrawTextW(hdc : HDC, lpchtext : windows_core::PCWSTR, cchtext : i32, lprc : *mut super::super::Foundation:: RECT, format : DRAW_TEXT_FORMAT) -> i32);
    unsafe { DrawTextW(hdc, core::mem::transmute(lpchtext.as_ptr()), lpchtext.len().try_into().unwrap(), lprc as _, format) }
}
#[inline]
pub unsafe fn Ellipse(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn Ellipse(hdc : HDC, left : i32, top : i32, right : i32, bottom : i32) -> windows_core::BOOL);
    unsafe { Ellipse(hdc, left, top, right, bottom) }
}
#[inline]
pub unsafe fn EndPaint(hwnd: super::super::Foundation::HWND, lppaint: *const PAINTSTRUCT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn EndPaint(hwnd : super::super::Foundation:: HWND, lppaint : *const PAINTSTRUCT) -> windows_core::BOOL);
    unsafe { EndPaint(hwnd, lppaint) }
}
#[inline]
pub unsafe fn EndPath(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EndPath(hdc : HDC) -> windows_core::BOOL);
    unsafe { EndPath(hdc) }
}
#[inline]
pub unsafe fn EnumDisplayDevicesA<P0>(lpdevice: P0, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEA, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("user32.dll" "system" fn EnumDisplayDevicesA(lpdevice : windows_core::PCSTR, idevnum : u32, lpdisplaydevice : *mut DISPLAY_DEVICEA, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDisplayDevicesA(lpdevice.param().abi(), idevnum, lpdisplaydevice as _, dwflags) }
}
#[inline]
pub unsafe fn EnumDisplayDevicesW<P0>(lpdevice: P0, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEW, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("user32.dll" "system" fn EnumDisplayDevicesW(lpdevice : windows_core::PCWSTR, idevnum : u32, lpdisplaydevice : *mut DISPLAY_DEVICEW, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDisplayDevicesW(lpdevice.param().abi(), idevnum, lpdisplaydevice as _, dwflags) }
}
#[inline]
pub unsafe fn EnumDisplayMonitors(hdc: Option<HDC>, lprcclip: Option<*const super::super::Foundation::RECT>, lpfnenum: MONITORENUMPROC, dwdata: super::super::Foundation::LPARAM) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn EnumDisplayMonitors(hdc : HDC, lprcclip : *const super::super::Foundation:: RECT, lpfnenum : MONITORENUMPROC, dwdata : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumDisplayMonitors(hdc.unwrap_or(core::mem::zeroed()) as _, lprcclip.unwrap_or(core::mem::zeroed()) as _, lpfnenum, dwdata) }
}
#[inline]
pub unsafe fn EnumDisplaySettingsA<P0>(lpszdevicename: P0, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("user32.dll" "system" fn EnumDisplaySettingsA(lpszdevicename : windows_core::PCSTR, imodenum : ENUM_DISPLAY_SETTINGS_MODE, lpdevmode : *mut DEVMODEA) -> windows_core::BOOL);
    unsafe { EnumDisplaySettingsA(lpszdevicename.param().abi(), imodenum, lpdevmode as _) }
}
#[inline]
pub unsafe fn EnumDisplaySettingsExA<P0>(lpszdevicename: P0, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA, dwflags: ENUM_DISPLAY_SETTINGS_FLAGS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("user32.dll" "system" fn EnumDisplaySettingsExA(lpszdevicename : windows_core::PCSTR, imodenum : ENUM_DISPLAY_SETTINGS_MODE, lpdevmode : *mut DEVMODEA, dwflags : ENUM_DISPLAY_SETTINGS_FLAGS) -> windows_core::BOOL);
    unsafe { EnumDisplaySettingsExA(lpszdevicename.param().abi(), imodenum, lpdevmode as _, dwflags) }
}
#[inline]
pub unsafe fn EnumDisplaySettingsExW<P0>(lpszdevicename: P0, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW, dwflags: ENUM_DISPLAY_SETTINGS_FLAGS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("user32.dll" "system" fn EnumDisplaySettingsExW(lpszdevicename : windows_core::PCWSTR, imodenum : ENUM_DISPLAY_SETTINGS_MODE, lpdevmode : *mut DEVMODEW, dwflags : ENUM_DISPLAY_SETTINGS_FLAGS) -> windows_core::BOOL);
    unsafe { EnumDisplaySettingsExW(lpszdevicename.param().abi(), imodenum, lpdevmode as _, dwflags) }
}
#[inline]
pub unsafe fn EnumDisplaySettingsW<P0>(lpszdevicename: P0, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("user32.dll" "system" fn EnumDisplaySettingsW(lpszdevicename : windows_core::PCWSTR, imodenum : ENUM_DISPLAY_SETTINGS_MODE, lpdevmode : *mut DEVMODEW) -> windows_core::BOOL);
    unsafe { EnumDisplaySettingsW(lpszdevicename.param().abi(), imodenum, lpdevmode as _) }
}
#[inline]
pub unsafe fn EnumEnhMetaFile(hdc: Option<HDC>, hmf: HENHMETAFILE, proc: ENHMFENUMPROC, param3: Option<*const core::ffi::c_void>, lprect: Option<*const super::super::Foundation::RECT>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EnumEnhMetaFile(hdc : HDC, hmf : HENHMETAFILE, proc : ENHMFENUMPROC, param3 : *const core::ffi::c_void, lprect : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { EnumEnhMetaFile(hdc.unwrap_or(core::mem::zeroed()) as _, hmf, proc, param3.unwrap_or(core::mem::zeroed()) as _, lprect.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnumFontFamiliesA<P1>(hdc: HDC, lplogfont: P1, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesA(hdc : HDC, lplogfont : windows_core::PCSTR, lpproc : FONTENUMPROCA, lparam : super::super::Foundation:: LPARAM) -> i32);
    unsafe { EnumFontFamiliesA(hdc, lplogfont.param().abi(), lpproc, lparam) }
}
#[inline]
pub unsafe fn EnumFontFamiliesExA(hdc: HDC, lplogfont: *const LOGFONTA, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesExA(hdc : HDC, lplogfont : *const LOGFONTA, lpproc : FONTENUMPROCA, lparam : super::super::Foundation:: LPARAM, dwflags : u32) -> i32);
    unsafe { EnumFontFamiliesExA(hdc, lplogfont, lpproc, lparam, dwflags) }
}
#[inline]
pub unsafe fn EnumFontFamiliesExW(hdc: HDC, lplogfont: *const LOGFONTW, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesExW(hdc : HDC, lplogfont : *const LOGFONTW, lpproc : FONTENUMPROCW, lparam : super::super::Foundation:: LPARAM, dwflags : u32) -> i32);
    unsafe { EnumFontFamiliesExW(hdc, lplogfont, lpproc, lparam, dwflags) }
}
#[inline]
pub unsafe fn EnumFontFamiliesW<P1>(hdc: HDC, lplogfont: P1, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesW(hdc : HDC, lplogfont : windows_core::PCWSTR, lpproc : FONTENUMPROCW, lparam : super::super::Foundation:: LPARAM) -> i32);
    unsafe { EnumFontFamiliesW(hdc, lplogfont.param().abi(), lpproc, lparam) }
}
#[inline]
pub unsafe fn EnumFontsA<P1>(hdc: HDC, lplogfont: P1, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn EnumFontsA(hdc : HDC, lplogfont : windows_core::PCSTR, lpproc : FONTENUMPROCA, lparam : super::super::Foundation:: LPARAM) -> i32);
    unsafe { EnumFontsA(hdc, lplogfont.param().abi(), lpproc, lparam) }
}
#[inline]
pub unsafe fn EnumFontsW<P1>(hdc: HDC, lplogfont: P1, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn EnumFontsW(hdc : HDC, lplogfont : windows_core::PCWSTR, lpproc : FONTENUMPROCW, lparam : super::super::Foundation:: LPARAM) -> i32);
    unsafe { EnumFontsW(hdc, lplogfont.param().abi(), lpproc, lparam) }
}
#[inline]
pub unsafe fn EnumMetaFile(hdc: HDC, hmf: HMETAFILE, proc: MFENUMPROC, param3: Option<super::super::Foundation::LPARAM>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EnumMetaFile(hdc : HDC, hmf : HMETAFILE, proc : MFENUMPROC, param3 : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { EnumMetaFile(hdc, hmf, proc, param3.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnumObjects(hdc: HDC, ntype: OBJ_TYPE, lpfunc: GOBJENUMPROC, lparam: super::super::Foundation::LPARAM) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn EnumObjects(hdc : HDC, ntype : OBJ_TYPE, lpfunc : GOBJENUMPROC, lparam : super::super::Foundation:: LPARAM) -> i32);
    unsafe { EnumObjects(hdc, ntype, lpfunc, lparam) }
}
#[inline]
pub unsafe fn EqualRect(lprc1: *const super::super::Foundation::RECT, lprc2: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn EqualRect(lprc1 : *const super::super::Foundation:: RECT, lprc2 : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { EqualRect(lprc1, lprc2) }
}
#[inline]
pub unsafe fn EqualRgn(hrgn1: HRGN, hrgn2: HRGN) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EqualRgn(hrgn1 : HRGN, hrgn2 : HRGN) -> windows_core::BOOL);
    unsafe { EqualRgn(hrgn1, hrgn2) }
}
#[inline]
pub unsafe fn ExcludeClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn ExcludeClipRect(hdc : HDC, left : i32, top : i32, right : i32, bottom : i32) -> GDI_REGION_TYPE);
    unsafe { ExcludeClipRect(hdc, left, top, right, bottom) }
}
#[inline]
pub unsafe fn ExcludeUpdateRgn(hdc: HDC, hwnd: super::super::Foundation::HWND) -> i32 {
    windows_link::link!("user32.dll" "system" fn ExcludeUpdateRgn(hdc : HDC, hwnd : super::super::Foundation:: HWND) -> i32);
    unsafe { ExcludeUpdateRgn(hdc, hwnd) }
}
#[inline]
pub unsafe fn ExtCreatePen(ipenstyle: PEN_STYLE, cwidth: u32, plbrush: *const LOGBRUSH, pstyle: Option<&[u32]>) -> HPEN {
    windows_link::link!("gdi32.dll" "system" fn ExtCreatePen(ipenstyle : u32, cwidth : u32, plbrush : *const LOGBRUSH, cstyle : u32, pstyle : *const u32) -> HPEN);
    unsafe { ExtCreatePen(ipenstyle.0 as _, cwidth, plbrush, pstyle.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pstyle.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn ExtCreateRegion(lpx: Option<*const XFORM>, ncount: u32, lpdata: *const RGNDATA) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn ExtCreateRegion(lpx : *const XFORM, ncount : u32, lpdata : *const RGNDATA) -> HRGN);
    unsafe { ExtCreateRegion(lpx.unwrap_or(core::mem::zeroed()) as _, ncount, lpdata) }
}
#[inline]
pub unsafe fn ExtFloodFill(hdc: HDC, x: i32, y: i32, color: super::super::Foundation::COLORREF, r#type: EXT_FLOOD_FILL_TYPE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn ExtFloodFill(hdc : HDC, x : i32, y : i32, color : super::super::Foundation:: COLORREF, r#type : EXT_FLOOD_FILL_TYPE) -> windows_core::BOOL);
    unsafe { ExtFloodFill(hdc, x, y, color, r#type) }
}
#[inline]
pub unsafe fn ExtSelectClipRgn(hdc: HDC, hrgn: Option<HRGN>, mode: RGN_COMBINE_MODE) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn ExtSelectClipRgn(hdc : HDC, hrgn : HRGN, mode : RGN_COMBINE_MODE) -> GDI_REGION_TYPE);
    unsafe { ExtSelectClipRgn(hdc, hrgn.unwrap_or(core::mem::zeroed()) as _, mode) }
}
#[inline]
pub unsafe fn ExtTextOutA<P5>(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: Option<*const super::super::Foundation::RECT>, lpstring: P5, c: u32, lpdx: Option<*const i32>) -> windows_core::BOOL
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn ExtTextOutA(hdc : HDC, x : i32, y : i32, options : ETO_OPTIONS, lprect : *const super::super::Foundation:: RECT, lpstring : windows_core::PCSTR, c : u32, lpdx : *const i32) -> windows_core::BOOL);
    unsafe { ExtTextOutA(hdc, x, y, options, lprect.unwrap_or(core::mem::zeroed()) as _, lpstring.param().abi(), c, lpdx.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ExtTextOutW<P5>(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: Option<*const super::super::Foundation::RECT>, lpstring: P5, c: u32, lpdx: Option<*const i32>) -> windows_core::BOOL
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn ExtTextOutW(hdc : HDC, x : i32, y : i32, options : ETO_OPTIONS, lprect : *const super::super::Foundation:: RECT, lpstring : windows_core::PCWSTR, c : u32, lpdx : *const i32) -> windows_core::BOOL);
    unsafe { ExtTextOutW(hdc, x, y, options, lprect.unwrap_or(core::mem::zeroed()) as _, lpstring.param().abi(), c, lpdx.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FillPath(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn FillPath(hdc : HDC) -> windows_core::BOOL);
    unsafe { FillPath(hdc) }
}
#[inline]
pub unsafe fn FillRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32 {
    windows_link::link!("user32.dll" "system" fn FillRect(hdc : HDC, lprc : *const super::super::Foundation:: RECT, hbr : HBRUSH) -> i32);
    unsafe { FillRect(hdc, lprc, hbr) }
}
#[inline]
pub unsafe fn FillRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn FillRgn(hdc : HDC, hrgn : HRGN, hbr : HBRUSH) -> windows_core::BOOL);
    unsafe { FillRgn(hdc, hrgn, hbr) }
}
#[inline]
pub unsafe fn FixBrushOrgEx(hdc: HDC, x: i32, y: i32, ptl: Option<*const super::super::Foundation::POINT>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn FixBrushOrgEx(hdc : HDC, x : i32, y : i32, ptl : *const super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { FixBrushOrgEx(hdc, x, y, ptl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FlattenPath(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn FlattenPath(hdc : HDC) -> windows_core::BOOL);
    unsafe { FlattenPath(hdc) }
}
#[inline]
pub unsafe fn FloodFill(hdc: HDC, x: i32, y: i32, color: super::super::Foundation::COLORREF) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn FloodFill(hdc : HDC, x : i32, y : i32, color : super::super::Foundation:: COLORREF) -> windows_core::BOOL);
    unsafe { FloodFill(hdc, x, y, color) }
}
#[inline]
pub unsafe fn FrameRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32 {
    windows_link::link!("user32.dll" "system" fn FrameRect(hdc : HDC, lprc : *const super::super::Foundation:: RECT, hbr : HBRUSH) -> i32);
    unsafe { FrameRect(hdc, lprc, hbr) }
}
#[inline]
pub unsafe fn FrameRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH, w: i32, h: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn FrameRgn(hdc : HDC, hrgn : HRGN, hbr : HBRUSH, w : i32, h : i32) -> windows_core::BOOL);
    unsafe { FrameRgn(hdc, hrgn, hbr, w, h) }
}
#[inline]
pub unsafe fn GdiAlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GdiAlphaBlend(hdcdest : HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, ftn : BLENDFUNCTION) -> windows_core::BOOL);
    unsafe { GdiAlphaBlend(hdcdest, xorigindest, yorigindest, wdest, hdest, hdcsrc, xoriginsrc, yoriginsrc, wsrc, hsrc, core::mem::transmute(ftn)) }
}
#[inline]
pub unsafe fn GdiComment(hdc: HDC, lpdata: &[u8]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GdiComment(hdc : HDC, nsize : u32, lpdata : *const u8) -> windows_core::BOOL);
    unsafe { GdiComment(hdc, lpdata.len().try_into().unwrap(), core::mem::transmute(lpdata.as_ptr())) }
}
#[inline]
pub unsafe fn GdiFlush() -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GdiFlush() -> windows_core::BOOL);
    unsafe { GdiFlush() }
}
#[inline]
pub unsafe fn GdiGetBatchLimit() -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GdiGetBatchLimit() -> u32);
    unsafe { GdiGetBatchLimit() }
}
#[inline]
pub unsafe fn GdiGradientFill(hdc: HDC, pvertex: &[TRIVERTEX], pmesh: *const core::ffi::c_void, ncount: u32, ulmode: GRADIENT_FILL) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GdiGradientFill(hdc : HDC, pvertex : *const TRIVERTEX, nvertex : u32, pmesh : *const core::ffi::c_void, ncount : u32, ulmode : GRADIENT_FILL) -> windows_core::BOOL);
    unsafe { GdiGradientFill(hdc, core::mem::transmute(pvertex.as_ptr()), pvertex.len().try_into().unwrap(), pmesh, ncount, ulmode) }
}
#[inline]
pub unsafe fn GdiSetBatchLimit(dw: u32) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GdiSetBatchLimit(dw : u32) -> u32);
    unsafe { GdiSetBatchLimit(dw) }
}
#[inline]
pub unsafe fn GdiTransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GdiTransparentBlt(hdcdest : HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, crtransparent : u32) -> windows_core::BOOL);
    unsafe { GdiTransparentBlt(hdcdest, xorigindest, yorigindest, wdest, hdest, hdcsrc, xoriginsrc, yoriginsrc, wsrc, hsrc, crtransparent) }
}
#[inline]
pub unsafe fn GetArcDirection(hdc: HDC) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetArcDirection(hdc : HDC) -> i32);
    unsafe { GetArcDirection(hdc) }
}
#[inline]
pub unsafe fn GetAspectRatioFilterEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetAspectRatioFilterEx(hdc : HDC, lpsize : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetAspectRatioFilterEx(hdc, lpsize as _) }
}
#[inline]
pub unsafe fn GetBitmapBits(hbit: HBITMAP, cb: i32, lpvbits: *mut core::ffi::c_void) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetBitmapBits(hbit : HBITMAP, cb : i32, lpvbits : *mut core::ffi::c_void) -> i32);
    unsafe { GetBitmapBits(hbit, cb, lpvbits as _) }
}
#[inline]
pub unsafe fn GetBitmapDimensionEx(hbit: HBITMAP, lpsize: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetBitmapDimensionEx(hbit : HBITMAP, lpsize : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetBitmapDimensionEx(hbit, lpsize as _) }
}
#[inline]
pub unsafe fn GetBkColor(hdc: HDC) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn GetBkColor(hdc : HDC) -> super::super::Foundation:: COLORREF);
    unsafe { GetBkColor(hdc) }
}
#[inline]
pub unsafe fn GetBkMode(hdc: HDC) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetBkMode(hdc : HDC) -> i32);
    unsafe { GetBkMode(hdc) }
}
#[inline]
pub unsafe fn GetBoundsRect(hdc: HDC, lprect: *mut super::super::Foundation::RECT, flags: u32) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetBoundsRect(hdc : HDC, lprect : *mut super::super::Foundation:: RECT, flags : u32) -> u32);
    unsafe { GetBoundsRect(hdc, lprect as _, flags) }
}
#[inline]
pub unsafe fn GetBrushOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetBrushOrgEx(hdc : HDC, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { GetBrushOrgEx(hdc, lppt as _) }
}
#[inline]
pub unsafe fn GetCharABCWidthsA(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsA(hdc : HDC, wfirst : u32, wlast : u32, lpabc : *mut ABC) -> windows_core::BOOL);
    unsafe { GetCharABCWidthsA(hdc, wfirst, wlast, lpabc as _) }
}
#[inline]
pub unsafe fn GetCharABCWidthsFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsFloatA(hdc : HDC, ifirst : u32, ilast : u32, lpabc : *mut ABCFLOAT) -> windows_core::BOOL);
    unsafe { GetCharABCWidthsFloatA(hdc, ifirst, ilast, lpabc as _) }
}
#[inline]
pub unsafe fn GetCharABCWidthsFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsFloatW(hdc : HDC, ifirst : u32, ilast : u32, lpabc : *mut ABCFLOAT) -> windows_core::BOOL);
    unsafe { GetCharABCWidthsFloatW(hdc, ifirst, ilast, lpabc as _) }
}
#[inline]
pub unsafe fn GetCharABCWidthsI(hdc: HDC, gifirst: u32, cgi: u32, pgi: Option<*const u16>, pabc: *mut ABC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsI(hdc : HDC, gifirst : u32, cgi : u32, pgi : *const u16, pabc : *mut ABC) -> windows_core::BOOL);
    unsafe { GetCharABCWidthsI(hdc, gifirst, cgi, pgi.unwrap_or(core::mem::zeroed()) as _, pabc as _) }
}
#[inline]
pub unsafe fn GetCharABCWidthsW(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsW(hdc : HDC, wfirst : u32, wlast : u32, lpabc : *mut ABC) -> windows_core::BOOL);
    unsafe { GetCharABCWidthsW(hdc, wfirst, wlast, lpabc as _) }
}
#[inline]
pub unsafe fn GetCharWidth32A(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharWidth32A(hdc : HDC, ifirst : u32, ilast : u32, lpbuffer : *mut i32) -> windows_core::BOOL);
    unsafe { GetCharWidth32A(hdc, ifirst, ilast, lpbuffer as _) }
}
#[inline]
pub unsafe fn GetCharWidth32W(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharWidth32W(hdc : HDC, ifirst : u32, ilast : u32, lpbuffer : *mut i32) -> windows_core::BOOL);
    unsafe { GetCharWidth32W(hdc, ifirst, ilast, lpbuffer as _) }
}
#[inline]
pub unsafe fn GetCharWidthA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharWidthA(hdc : HDC, ifirst : u32, ilast : u32, lpbuffer : *mut i32) -> windows_core::BOOL);
    unsafe { GetCharWidthA(hdc, ifirst, ilast, lpbuffer as _) }
}
#[inline]
pub unsafe fn GetCharWidthFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharWidthFloatA(hdc : HDC, ifirst : u32, ilast : u32, lpbuffer : *mut f32) -> windows_core::BOOL);
    unsafe { GetCharWidthFloatA(hdc, ifirst, ilast, lpbuffer as _) }
}
#[inline]
pub unsafe fn GetCharWidthFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharWidthFloatW(hdc : HDC, ifirst : u32, ilast : u32, lpbuffer : *mut f32) -> windows_core::BOOL);
    unsafe { GetCharWidthFloatW(hdc, ifirst, ilast, lpbuffer as _) }
}
#[inline]
pub unsafe fn GetCharWidthI(hdc: HDC, gifirst: u32, cgi: u32, pgi: Option<*const u16>, piwidths: *mut i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharWidthI(hdc : HDC, gifirst : u32, cgi : u32, pgi : *const u16, piwidths : *mut i32) -> windows_core::BOOL);
    unsafe { GetCharWidthI(hdc, gifirst, cgi, pgi.unwrap_or(core::mem::zeroed()) as _, piwidths as _) }
}
#[inline]
pub unsafe fn GetCharWidthW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCharWidthW(hdc : HDC, ifirst : u32, ilast : u32, lpbuffer : *mut i32) -> windows_core::BOOL);
    unsafe { GetCharWidthW(hdc, ifirst, ilast, lpbuffer as _) }
}
#[inline]
pub unsafe fn GetCharacterPlacementA(hdc: HDC, lpstring: &[u8], nmexextent: i32, lpresults: *mut GCP_RESULTSA, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetCharacterPlacementA(hdc : HDC, lpstring : windows_core::PCSTR, ncount : i32, nmexextent : i32, lpresults : *mut GCP_RESULTSA, dwflags : GET_CHARACTER_PLACEMENT_FLAGS) -> u32);
    unsafe { GetCharacterPlacementA(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), nmexextent, lpresults as _, dwflags) }
}
#[inline]
pub unsafe fn GetCharacterPlacementW(hdc: HDC, lpstring: &[u16], nmexextent: i32, lpresults: *mut GCP_RESULTSW, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetCharacterPlacementW(hdc : HDC, lpstring : windows_core::PCWSTR, ncount : i32, nmexextent : i32, lpresults : *mut GCP_RESULTSW, dwflags : GET_CHARACTER_PLACEMENT_FLAGS) -> u32);
    unsafe { GetCharacterPlacementW(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), nmexextent, lpresults as _, dwflags) }
}
#[inline]
pub unsafe fn GetClipBox(hdc: HDC, lprect: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn GetClipBox(hdc : HDC, lprect : *mut super::super::Foundation:: RECT) -> GDI_REGION_TYPE);
    unsafe { GetClipBox(hdc, lprect as _) }
}
#[inline]
pub unsafe fn GetClipRgn(hdc: HDC, hrgn: HRGN) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetClipRgn(hdc : HDC, hrgn : HRGN) -> i32);
    unsafe { GetClipRgn(hdc, hrgn) }
}
#[inline]
pub unsafe fn GetColorAdjustment(hdc: HDC, lpca: *mut COLORADJUSTMENT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetColorAdjustment(hdc : HDC, lpca : *mut COLORADJUSTMENT) -> windows_core::BOOL);
    unsafe { GetColorAdjustment(hdc, lpca as _) }
}
#[inline]
pub unsafe fn GetCurrentObject(hdc: HDC, r#type: OBJ_TYPE) -> HGDIOBJ {
    windows_link::link!("gdi32.dll" "system" fn GetCurrentObject(hdc : HDC, r#type : u32) -> HGDIOBJ);
    unsafe { GetCurrentObject(hdc, r#type.0 as _) }
}
#[inline]
pub unsafe fn GetCurrentPositionEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetCurrentPositionEx(hdc : HDC, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { GetCurrentPositionEx(hdc, lppt as _) }
}
#[inline]
pub unsafe fn GetDC(hwnd: Option<super::super::Foundation::HWND>) -> HDC {
    windows_link::link!("user32.dll" "system" fn GetDC(hwnd : super::super::Foundation:: HWND) -> HDC);
    unsafe { GetDC(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetDCBrushColor(hdc: HDC) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn GetDCBrushColor(hdc : HDC) -> super::super::Foundation:: COLORREF);
    unsafe { GetDCBrushColor(hdc) }
}
#[inline]
pub unsafe fn GetDCEx(hwnd: Option<super::super::Foundation::HWND>, hrgnclip: Option<HRGN>, flags: GET_DCX_FLAGS) -> HDC {
    windows_link::link!("user32.dll" "system" fn GetDCEx(hwnd : super::super::Foundation:: HWND, hrgnclip : HRGN, flags : GET_DCX_FLAGS) -> HDC);
    unsafe { GetDCEx(hwnd.unwrap_or(core::mem::zeroed()) as _, hrgnclip.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn GetDCOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetDCOrgEx(hdc : HDC, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { GetDCOrgEx(hdc, lppt as _) }
}
#[inline]
pub unsafe fn GetDCPenColor(hdc: HDC) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn GetDCPenColor(hdc : HDC) -> super::super::Foundation:: COLORREF);
    unsafe { GetDCPenColor(hdc) }
}
#[inline]
pub unsafe fn GetDIBColorTable(hdc: HDC, istart: u32, prgbq: &mut [RGBQUAD]) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetDIBColorTable(hdc : HDC, istart : u32, centries : u32, prgbq : *mut RGBQUAD) -> u32);
    unsafe { GetDIBColorTable(hdc, istart, prgbq.len().try_into().unwrap(), core::mem::transmute(prgbq.as_ptr())) }
}
#[inline]
pub unsafe fn GetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpvbits: Option<*mut core::ffi::c_void>, lpbmi: *mut BITMAPINFO, usage: DIB_USAGE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetDIBits(hdc : HDC, hbm : HBITMAP, start : u32, clines : u32, lpvbits : *mut core::ffi::c_void, lpbmi : *mut BITMAPINFO, usage : DIB_USAGE) -> i32);
    unsafe { GetDIBits(hdc, hbm, start, clines, lpvbits.unwrap_or(core::mem::zeroed()) as _, lpbmi as _, usage) }
}
#[inline]
pub unsafe fn GetDeviceCaps(hdc: Option<HDC>, index: GET_DEVICE_CAPS_INDEX) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetDeviceCaps(hdc : HDC, index : i32) -> i32);
    unsafe { GetDeviceCaps(hdc.unwrap_or(core::mem::zeroed()) as _, index.0 as _) }
}
#[inline]
pub unsafe fn GetEnhMetaFileA<P0>(lpname: P0) -> HENHMETAFILE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileA(lpname : windows_core::PCSTR) -> HENHMETAFILE);
    unsafe { GetEnhMetaFileA(lpname.param().abi()) }
}
#[inline]
pub unsafe fn GetEnhMetaFileBits(hemf: HENHMETAFILE, lpdata: Option<&mut [u8]>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileBits(hemf : HENHMETAFILE, nsize : u32, lpdata : *mut u8) -> u32);
    unsafe { GetEnhMetaFileBits(hemf, lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetEnhMetaFileDescriptionA(hemf: HENHMETAFILE, lpdescription: Option<&mut [u8]>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileDescriptionA(hemf : HENHMETAFILE, cchbuffer : u32, lpdescription : windows_core::PSTR) -> u32);
    unsafe { GetEnhMetaFileDescriptionA(hemf, lpdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetEnhMetaFileDescriptionW(hemf: HENHMETAFILE, lpdescription: Option<&mut [u16]>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileDescriptionW(hemf : HENHMETAFILE, cchbuffer : u32, lpdescription : windows_core::PWSTR) -> u32);
    unsafe { GetEnhMetaFileDescriptionW(hemf, lpdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetEnhMetaFileHeader(hemf: HENHMETAFILE, nsize: u32, lpenhmetaheader: Option<*mut ENHMETAHEADER>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileHeader(hemf : HENHMETAFILE, nsize : u32, lpenhmetaheader : *mut ENHMETAHEADER) -> u32);
    unsafe { GetEnhMetaFileHeader(hemf, nsize, lpenhmetaheader.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetEnhMetaFilePaletteEntries(hemf: HENHMETAFILE, lppaletteentries: Option<&mut [PALETTEENTRY]>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFilePaletteEntries(hemf : HENHMETAFILE, nnumentries : u32, lppaletteentries : *mut PALETTEENTRY) -> u32);
    unsafe { GetEnhMetaFilePaletteEntries(hemf, lppaletteentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lppaletteentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetEnhMetaFileW<P0>(lpname: P0) -> HENHMETAFILE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileW(lpname : windows_core::PCWSTR) -> HENHMETAFILE);
    unsafe { GetEnhMetaFileW(lpname.param().abi()) }
}
#[inline]
pub unsafe fn GetFontData(hdc: HDC, dwtable: u32, dwoffset: u32, pvbuffer: Option<*mut core::ffi::c_void>, cjbuffer: u32) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetFontData(hdc : HDC, dwtable : u32, dwoffset : u32, pvbuffer : *mut core::ffi::c_void, cjbuffer : u32) -> u32);
    unsafe { GetFontData(hdc, dwtable, dwoffset, pvbuffer.unwrap_or(core::mem::zeroed()) as _, cjbuffer) }
}
#[inline]
pub unsafe fn GetFontLanguageInfo(hdc: HDC) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetFontLanguageInfo(hdc : HDC) -> u32);
    unsafe { GetFontLanguageInfo(hdc) }
}
#[inline]
pub unsafe fn GetFontUnicodeRanges(hdc: HDC, lpgs: Option<*mut GLYPHSET>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetFontUnicodeRanges(hdc : HDC, lpgs : *mut GLYPHSET) -> u32);
    unsafe { GetFontUnicodeRanges(hdc, lpgs.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetGlyphIndicesA<P1>(hdc: HDC, lpstr: P1, c: i32, pgi: *mut u16, fl: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn GetGlyphIndicesA(hdc : HDC, lpstr : windows_core::PCSTR, c : i32, pgi : *mut u16, fl : u32) -> u32);
    unsafe { GetGlyphIndicesA(hdc, lpstr.param().abi(), c, pgi as _, fl) }
}
#[inline]
pub unsafe fn GetGlyphIndicesW<P1>(hdc: HDC, lpstr: P1, c: i32, pgi: *mut u16, fl: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn GetGlyphIndicesW(hdc : HDC, lpstr : windows_core::PCWSTR, c : i32, pgi : *mut u16, fl : u32) -> u32);
    unsafe { GetGlyphIndicesW(hdc, lpstr.param().abi(), c, pgi as _, fl) }
}
#[inline]
pub unsafe fn GetGlyphOutlineA(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: Option<*mut core::ffi::c_void>, lpmat2: *const MAT2) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetGlyphOutlineA(hdc : HDC, uchar : u32, fuformat : GET_GLYPH_OUTLINE_FORMAT, lpgm : *mut GLYPHMETRICS, cjbuffer : u32, pvbuffer : *mut core::ffi::c_void, lpmat2 : *const MAT2) -> u32);
    unsafe { GetGlyphOutlineA(hdc, uchar, fuformat, lpgm as _, cjbuffer, pvbuffer.unwrap_or(core::mem::zeroed()) as _, lpmat2) }
}
#[inline]
pub unsafe fn GetGlyphOutlineW(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: Option<*mut core::ffi::c_void>, lpmat2: *const MAT2) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetGlyphOutlineW(hdc : HDC, uchar : u32, fuformat : GET_GLYPH_OUTLINE_FORMAT, lpgm : *mut GLYPHMETRICS, cjbuffer : u32, pvbuffer : *mut core::ffi::c_void, lpmat2 : *const MAT2) -> u32);
    unsafe { GetGlyphOutlineW(hdc, uchar, fuformat, lpgm as _, cjbuffer, pvbuffer.unwrap_or(core::mem::zeroed()) as _, lpmat2) }
}
#[inline]
pub unsafe fn GetGraphicsMode(hdc: HDC) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetGraphicsMode(hdc : HDC) -> i32);
    unsafe { GetGraphicsMode(hdc) }
}
#[inline]
pub unsafe fn GetKerningPairsA(hdc: HDC, lpkernpair: Option<&mut [KERNINGPAIR]>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetKerningPairsA(hdc : HDC, npairs : u32, lpkernpair : *mut KERNINGPAIR) -> u32);
    unsafe { GetKerningPairsA(hdc, lpkernpair.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpkernpair.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetKerningPairsW(hdc: HDC, lpkernpair: Option<&mut [KERNINGPAIR]>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetKerningPairsW(hdc : HDC, npairs : u32, lpkernpair : *mut KERNINGPAIR) -> u32);
    unsafe { GetKerningPairsW(hdc, lpkernpair.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpkernpair.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetLayout(hdc: HDC) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetLayout(hdc : HDC) -> u32);
    unsafe { GetLayout(hdc) }
}
#[inline]
pub unsafe fn GetMapMode(hdc: HDC) -> HDC_MAP_MODE {
    windows_link::link!("gdi32.dll" "system" fn GetMapMode(hdc : HDC) -> HDC_MAP_MODE);
    unsafe { GetMapMode(hdc) }
}
#[inline]
pub unsafe fn GetMetaFileA<P0>(lpname: P0) -> HMETAFILE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn GetMetaFileA(lpname : windows_core::PCSTR) -> HMETAFILE);
    unsafe { GetMetaFileA(lpname.param().abi()) }
}
#[inline]
pub unsafe fn GetMetaFileBitsEx(hmf: HMETAFILE, cbbuffer: u32, lpdata: Option<*mut core::ffi::c_void>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetMetaFileBitsEx(hmf : HMETAFILE, cbbuffer : u32, lpdata : *mut core::ffi::c_void) -> u32);
    unsafe { GetMetaFileBitsEx(hmf, cbbuffer, lpdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetMetaFileW<P0>(lpname: P0) -> HMETAFILE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn GetMetaFileW(lpname : windows_core::PCWSTR) -> HMETAFILE);
    unsafe { GetMetaFileW(lpname.param().abi()) }
}
#[inline]
pub unsafe fn GetMetaRgn(hdc: HDC, hrgn: HRGN) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetMetaRgn(hdc : HDC, hrgn : HRGN) -> i32);
    unsafe { GetMetaRgn(hdc, hrgn) }
}
#[inline]
pub unsafe fn GetMiterLimit(hdc: HDC, plimit: *mut f32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetMiterLimit(hdc : HDC, plimit : *mut f32) -> windows_core::BOOL);
    unsafe { GetMiterLimit(hdc, plimit as _) }
}
#[inline]
pub unsafe fn GetMonitorInfoA(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn GetMonitorInfoA(hmonitor : HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
    unsafe { GetMonitorInfoA(hmonitor, lpmi as _) }
}
#[inline]
pub unsafe fn GetMonitorInfoW(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn GetMonitorInfoW(hmonitor : HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
    unsafe { GetMonitorInfoW(hmonitor, lpmi as _) }
}
#[inline]
pub unsafe fn GetNearestColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn GetNearestColor(hdc : HDC, color : super::super::Foundation:: COLORREF) -> super::super::Foundation:: COLORREF);
    unsafe { GetNearestColor(hdc, color) }
}
#[inline]
pub unsafe fn GetNearestPaletteIndex(h: HPALETTE, color: super::super::Foundation::COLORREF) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetNearestPaletteIndex(h : HPALETTE, color : super::super::Foundation:: COLORREF) -> u32);
    unsafe { GetNearestPaletteIndex(h, color) }
}
#[inline]
pub unsafe fn GetObjectA(h: HGDIOBJ, c: i32, pv: Option<*mut core::ffi::c_void>) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetObjectA(h : HGDIOBJ, c : i32, pv : *mut core::ffi::c_void) -> i32);
    unsafe { GetObjectA(h, c, pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetObjectType(h: HGDIOBJ) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetObjectType(h : HGDIOBJ) -> u32);
    unsafe { GetObjectType(h) }
}
#[inline]
pub unsafe fn GetObjectW(h: HGDIOBJ, c: i32, pv: Option<*mut core::ffi::c_void>) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetObjectW(h : HGDIOBJ, c : i32, pv : *mut core::ffi::c_void) -> i32);
    unsafe { GetObjectW(h, c, pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetOutlineTextMetricsA(hdc: HDC, cjcopy: u32, potm: Option<*mut OUTLINETEXTMETRICA>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetOutlineTextMetricsA(hdc : HDC, cjcopy : u32, potm : *mut OUTLINETEXTMETRICA) -> u32);
    unsafe { GetOutlineTextMetricsA(hdc, cjcopy, potm.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetOutlineTextMetricsW(hdc: HDC, cjcopy: u32, potm: Option<*mut OUTLINETEXTMETRICW>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetOutlineTextMetricsW(hdc : HDC, cjcopy : u32, potm : *mut OUTLINETEXTMETRICW) -> u32);
    unsafe { GetOutlineTextMetricsW(hdc, cjcopy, potm.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPaletteEntries(hpal: HPALETTE, istart: u32, ppalentries: Option<&mut [PALETTEENTRY]>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetPaletteEntries(hpal : HPALETTE, istart : u32, centries : u32, ppalentries : *mut PALETTEENTRY) -> u32);
    unsafe { GetPaletteEntries(hpal, istart, ppalentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppalentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetPath(hdc: HDC, apt: Option<*mut super::super::Foundation::POINT>, aj: Option<*mut u8>, cpt: i32) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetPath(hdc : HDC, apt : *mut super::super::Foundation:: POINT, aj : *mut u8, cpt : i32) -> i32);
    unsafe { GetPath(hdc, apt.unwrap_or(core::mem::zeroed()) as _, aj.unwrap_or(core::mem::zeroed()) as _, cpt) }
}
#[inline]
pub unsafe fn GetPixel(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn GetPixel(hdc : HDC, x : i32, y : i32) -> super::super::Foundation:: COLORREF);
    unsafe { GetPixel(hdc, x, y) }
}
#[inline]
pub unsafe fn GetPolyFillMode(hdc: HDC) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetPolyFillMode(hdc : HDC) -> i32);
    unsafe { GetPolyFillMode(hdc) }
}
#[inline]
pub unsafe fn GetROP2(hdc: HDC) -> R2_MODE {
    windows_link::link!("gdi32.dll" "system" fn GetROP2(hdc : HDC) -> R2_MODE);
    unsafe { GetROP2(hdc) }
}
#[inline]
pub unsafe fn GetRandomRgn(hdc: HDC, hrgn: HRGN, i: i32) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetRandomRgn(hdc : HDC, hrgn : HRGN, i : i32) -> i32);
    unsafe { GetRandomRgn(hdc, hrgn, i) }
}
#[inline]
pub unsafe fn GetRasterizerCaps(lpraststat: *mut RASTERIZER_STATUS, cjbytes: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetRasterizerCaps(lpraststat : *mut RASTERIZER_STATUS, cjbytes : u32) -> windows_core::BOOL);
    unsafe { GetRasterizerCaps(lpraststat as _, cjbytes) }
}
#[inline]
pub unsafe fn GetRegionData(hrgn: HRGN, ncount: u32, lprgndata: Option<*mut RGNDATA>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetRegionData(hrgn : HRGN, ncount : u32, lprgndata : *mut RGNDATA) -> u32);
    unsafe { GetRegionData(hrgn, ncount, lprgndata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetRgnBox(hrgn: HRGN, lprc: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn GetRgnBox(hrgn : HRGN, lprc : *mut super::super::Foundation:: RECT) -> GDI_REGION_TYPE);
    unsafe { GetRgnBox(hrgn, lprc as _) }
}
#[inline]
pub unsafe fn GetStockObject(i: GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ {
    windows_link::link!("gdi32.dll" "system" fn GetStockObject(i : GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ);
    unsafe { GetStockObject(i) }
}
#[inline]
pub unsafe fn GetStretchBltMode(hdc: HDC) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetStretchBltMode(hdc : HDC) -> i32);
    unsafe { GetStretchBltMode(hdc) }
}
#[inline]
pub unsafe fn GetSysColor(nindex: SYS_COLOR_INDEX) -> u32 {
    windows_link::link!("user32.dll" "system" fn GetSysColor(nindex : SYS_COLOR_INDEX) -> u32);
    unsafe { GetSysColor(nindex) }
}
#[inline]
pub unsafe fn GetSysColorBrush(nindex: SYS_COLOR_INDEX) -> HBRUSH {
    windows_link::link!("user32.dll" "system" fn GetSysColorBrush(nindex : SYS_COLOR_INDEX) -> HBRUSH);
    unsafe { GetSysColorBrush(nindex) }
}
#[inline]
pub unsafe fn GetSystemPaletteEntries(hdc: HDC, istart: u32, ppalentries: Option<&mut [PALETTEENTRY]>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetSystemPaletteEntries(hdc : HDC, istart : u32, centries : u32, ppalentries : *mut PALETTEENTRY) -> u32);
    unsafe { GetSystemPaletteEntries(hdc, istart, ppalentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppalentries.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetSystemPaletteUse(hdc: HDC) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetSystemPaletteUse(hdc : HDC) -> u32);
    unsafe { GetSystemPaletteUse(hdc) }
}
#[inline]
pub unsafe fn GetTabbedTextExtentA(hdc: HDC, lpstring: &[u8], lpntabstoppositions: Option<&[i32]>) -> u32 {
    windows_link::link!("user32.dll" "system" fn GetTabbedTextExtentA(hdc : HDC, lpstring : windows_core::PCSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32) -> u32);
    unsafe { GetTabbedTextExtentA(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpntabstoppositions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpntabstoppositions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetTabbedTextExtentW(hdc: HDC, lpstring: &[u16], lpntabstoppositions: Option<&[i32]>) -> u32 {
    windows_link::link!("user32.dll" "system" fn GetTabbedTextExtentW(hdc : HDC, lpstring : windows_core::PCWSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32) -> u32);
    unsafe { GetTabbedTextExtentW(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpntabstoppositions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpntabstoppositions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetTextAlign(hdc: HDC) -> TEXT_ALIGN_OPTIONS {
    windows_link::link!("gdi32.dll" "system" fn GetTextAlign(hdc : HDC) -> TEXT_ALIGN_OPTIONS);
    unsafe { GetTextAlign(hdc) }
}
#[inline]
pub unsafe fn GetTextCharacterExtra(hdc: HDC) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetTextCharacterExtra(hdc : HDC) -> i32);
    unsafe { GetTextCharacterExtra(hdc) }
}
#[inline]
pub unsafe fn GetTextColor(hdc: HDC) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn GetTextColor(hdc : HDC) -> super::super::Foundation:: COLORREF);
    unsafe { GetTextColor(hdc) }
}
#[inline]
pub unsafe fn GetTextExtentExPointA<P1>(hdc: HDC, lpszstring: P1, cchstring: i32, nmaxextent: i32, lpnfit: Option<*mut i32>, lpndx: Option<*mut i32>, lpsize: *mut super::super::Foundation::SIZE) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointA(hdc : HDC, lpszstring : windows_core::PCSTR, cchstring : i32, nmaxextent : i32, lpnfit : *mut i32, lpndx : *mut i32, lpsize : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetTextExtentExPointA(hdc, lpszstring.param().abi(), cchstring, nmaxextent, lpnfit.unwrap_or(core::mem::zeroed()) as _, lpndx.unwrap_or(core::mem::zeroed()) as _, lpsize as _) }
}
#[inline]
pub unsafe fn GetTextExtentExPointI(hdc: HDC, lpwszstring: *const u16, cwchstring: i32, nmaxextent: i32, lpnfit: Option<*mut i32>, lpndx: Option<*mut i32>, lpsize: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointI(hdc : HDC, lpwszstring : *const u16, cwchstring : i32, nmaxextent : i32, lpnfit : *mut i32, lpndx : *mut i32, lpsize : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetTextExtentExPointI(hdc, lpwszstring, cwchstring, nmaxextent, lpnfit.unwrap_or(core::mem::zeroed()) as _, lpndx.unwrap_or(core::mem::zeroed()) as _, lpsize as _) }
}
#[inline]
pub unsafe fn GetTextExtentExPointW<P1>(hdc: HDC, lpszstring: P1, cchstring: i32, nmaxextent: i32, lpnfit: Option<*mut i32>, lpndx: Option<*mut i32>, lpsize: *mut super::super::Foundation::SIZE) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointW(hdc : HDC, lpszstring : windows_core::PCWSTR, cchstring : i32, nmaxextent : i32, lpnfit : *mut i32, lpndx : *mut i32, lpsize : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetTextExtentExPointW(hdc, lpszstring.param().abi(), cchstring, nmaxextent, lpnfit.unwrap_or(core::mem::zeroed()) as _, lpndx.unwrap_or(core::mem::zeroed()) as _, lpsize as _) }
}
#[inline]
pub unsafe fn GetTextExtentPoint32A(hdc: HDC, lpstring: &[u8], psizl: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetTextExtentPoint32A(hdc : HDC, lpstring : windows_core::PCSTR, c : i32, psizl : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetTextExtentPoint32A(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), psizl as _) }
}
#[inline]
pub unsafe fn GetTextExtentPoint32W(hdc: HDC, lpstring: &[u16], psizl: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetTextExtentPoint32W(hdc : HDC, lpstring : windows_core::PCWSTR, c : i32, psizl : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetTextExtentPoint32W(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), psizl as _) }
}
#[inline]
pub unsafe fn GetTextExtentPointA(hdc: HDC, lpstring: &[u8], lpsz: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointA(hdc : HDC, lpstring : windows_core::PCSTR, c : i32, lpsz : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetTextExtentPointA(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpsz as _) }
}
#[inline]
pub unsafe fn GetTextExtentPointI(hdc: HDC, pgiin: &[u16], psize: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointI(hdc : HDC, pgiin : *const u16, cgi : i32, psize : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetTextExtentPointI(hdc, core::mem::transmute(pgiin.as_ptr()), pgiin.len().try_into().unwrap(), psize as _) }
}
#[inline]
pub unsafe fn GetTextExtentPointW(hdc: HDC, lpstring: &[u16], lpsz: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointW(hdc : HDC, lpstring : windows_core::PCWSTR, c : i32, lpsz : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetTextExtentPointW(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpsz as _) }
}
#[inline]
pub unsafe fn GetTextFaceA(hdc: HDC, lpname: Option<&mut [u8]>) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetTextFaceA(hdc : HDC, c : i32, lpname : windows_core::PSTR) -> i32);
    unsafe { GetTextFaceA(hdc, lpname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetTextFaceW(hdc: HDC, lpname: Option<&mut [u16]>) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn GetTextFaceW(hdc : HDC, c : i32, lpname : windows_core::PWSTR) -> i32);
    unsafe { GetTextFaceW(hdc, lpname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn GetTextMetricsA(hdc: HDC, lptm: *mut TEXTMETRICA) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetTextMetricsA(hdc : HDC, lptm : *mut TEXTMETRICA) -> windows_core::BOOL);
    unsafe { GetTextMetricsA(hdc, lptm as _) }
}
#[inline]
pub unsafe fn GetTextMetricsW(hdc: HDC, lptm: *mut TEXTMETRICW) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetTextMetricsW(hdc : HDC, lptm : *mut TEXTMETRICW) -> windows_core::BOOL);
    unsafe { GetTextMetricsW(hdc, lptm as _) }
}
#[inline]
pub unsafe fn GetUpdateRect(hwnd: super::super::Foundation::HWND, lprect: Option<*mut super::super::Foundation::RECT>, berase: bool) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn GetUpdateRect(hwnd : super::super::Foundation:: HWND, lprect : *mut super::super::Foundation:: RECT, berase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetUpdateRect(hwnd, lprect.unwrap_or(core::mem::zeroed()) as _, berase.into()) }
}
#[inline]
pub unsafe fn GetUpdateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: bool) -> GDI_REGION_TYPE {
    windows_link::link!("user32.dll" "system" fn GetUpdateRgn(hwnd : super::super::Foundation:: HWND, hrgn : HRGN, berase : windows_core::BOOL) -> GDI_REGION_TYPE);
    unsafe { GetUpdateRgn(hwnd, hrgn, berase.into()) }
}
#[inline]
pub unsafe fn GetViewportExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetViewportExtEx(hdc : HDC, lpsize : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetViewportExtEx(hdc, lpsize as _) }
}
#[inline]
pub unsafe fn GetViewportOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetViewportOrgEx(hdc : HDC, lppoint : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { GetViewportOrgEx(hdc, lppoint as _) }
}
#[inline]
pub unsafe fn GetWinMetaFileBits(hemf: HENHMETAFILE, pdata16: Option<&mut [u8]>, imapmode: i32, hdcref: HDC) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn GetWinMetaFileBits(hemf : HENHMETAFILE, cbdata16 : u32, pdata16 : *mut u8, imapmode : i32, hdcref : HDC) -> u32);
    unsafe { GetWinMetaFileBits(hemf, pdata16.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pdata16.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), imapmode, hdcref) }
}
#[inline]
pub unsafe fn GetWindowDC(hwnd: Option<super::super::Foundation::HWND>) -> HDC {
    windows_link::link!("user32.dll" "system" fn GetWindowDC(hwnd : super::super::Foundation:: HWND) -> HDC);
    unsafe { GetWindowDC(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetWindowExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetWindowExtEx(hdc : HDC, lpsize : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { GetWindowExtEx(hdc, lpsize as _) }
}
#[inline]
pub unsafe fn GetWindowOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetWindowOrgEx(hdc : HDC, lppoint : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { GetWindowOrgEx(hdc, lppoint as _) }
}
#[inline]
pub unsafe fn GetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> GDI_REGION_TYPE {
    windows_link::link!("user32.dll" "system" fn GetWindowRgn(hwnd : super::super::Foundation:: HWND, hrgn : HRGN) -> GDI_REGION_TYPE);
    unsafe { GetWindowRgn(hwnd, hrgn) }
}
#[inline]
pub unsafe fn GetWindowRgnBox(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT) -> GDI_REGION_TYPE {
    windows_link::link!("user32.dll" "system" fn GetWindowRgnBox(hwnd : super::super::Foundation:: HWND, lprc : *mut super::super::Foundation:: RECT) -> GDI_REGION_TYPE);
    unsafe { GetWindowRgnBox(hwnd, lprc as _) }
}
#[inline]
pub unsafe fn GetWorldTransform(hdc: HDC, lpxf: *mut XFORM) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn GetWorldTransform(hdc : HDC, lpxf : *mut XFORM) -> windows_core::BOOL);
    unsafe { GetWorldTransform(hdc, lpxf as _) }
}
#[inline]
pub unsafe fn GradientFill(hdc: HDC, pvertex: &[TRIVERTEX], pmesh: *const core::ffi::c_void, nmesh: u32, ulmode: GRADIENT_FILL) -> windows_core::BOOL {
    windows_link::link!("msimg32.dll" "system" fn GradientFill(hdc : HDC, pvertex : *const TRIVERTEX, nvertex : u32, pmesh : *const core::ffi::c_void, nmesh : u32, ulmode : GRADIENT_FILL) -> windows_core::BOOL);
    unsafe { GradientFill(hdc, core::mem::transmute(pvertex.as_ptr()), pvertex.len().try_into().unwrap(), pmesh, nmesh, ulmode) }
}
#[inline]
pub unsafe fn GrayStringA(hdc: HDC, hbrush: Option<HBRUSH>, lpoutputfunc: GRAYSTRINGPROC, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn GrayStringA(hdc : HDC, hbrush : HBRUSH, lpoutputfunc : GRAYSTRINGPROC, lpdata : super::super::Foundation:: LPARAM, ncount : i32, x : i32, y : i32, nwidth : i32, nheight : i32) -> windows_core::BOOL);
    unsafe { GrayStringA(hdc, hbrush.unwrap_or(core::mem::zeroed()) as _, lpoutputfunc, lpdata, ncount, x, y, nwidth, nheight) }
}
#[inline]
pub unsafe fn GrayStringW(hdc: HDC, hbrush: Option<HBRUSH>, lpoutputfunc: GRAYSTRINGPROC, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn GrayStringW(hdc : HDC, hbrush : HBRUSH, lpoutputfunc : GRAYSTRINGPROC, lpdata : super::super::Foundation:: LPARAM, ncount : i32, x : i32, y : i32, nwidth : i32, nheight : i32) -> windows_core::BOOL);
    unsafe { GrayStringW(hdc, hbrush.unwrap_or(core::mem::zeroed()) as _, lpoutputfunc, lpdata, ncount, x, y, nwidth, nheight) }
}
#[inline]
pub unsafe fn InflateRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn InflateRect(lprc : *mut super::super::Foundation:: RECT, dx : i32, dy : i32) -> windows_core::BOOL);
    unsafe { InflateRect(lprc as _, dx, dy) }
}
#[inline]
pub unsafe fn IntersectClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn IntersectClipRect(hdc : HDC, left : i32, top : i32, right : i32, bottom : i32) -> GDI_REGION_TYPE);
    unsafe { IntersectClipRect(hdc, left, top, right, bottom) }
}
#[inline]
pub unsafe fn IntersectRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn IntersectRect(lprcdst : *mut super::super::Foundation:: RECT, lprcsrc1 : *const super::super::Foundation:: RECT, lprcsrc2 : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { IntersectRect(lprcdst as _, lprcsrc1, lprcsrc2) }
}
#[inline]
pub unsafe fn InvalidateRect(hwnd: Option<super::super::Foundation::HWND>, lprect: Option<*const super::super::Foundation::RECT>, berase: bool) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn InvalidateRect(hwnd : super::super::Foundation:: HWND, lprect : *const super::super::Foundation:: RECT, berase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { InvalidateRect(hwnd.unwrap_or(core::mem::zeroed()) as _, lprect.unwrap_or(core::mem::zeroed()) as _, berase.into()) }
}
#[inline]
pub unsafe fn InvalidateRgn(hwnd: super::super::Foundation::HWND, hrgn: Option<HRGN>, berase: bool) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn InvalidateRgn(hwnd : super::super::Foundation:: HWND, hrgn : HRGN, berase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { InvalidateRgn(hwnd, hrgn.unwrap_or(core::mem::zeroed()) as _, berase.into()) }
}
#[inline]
pub unsafe fn InvertRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn InvertRect(hdc : HDC, lprc : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { InvertRect(hdc, lprc) }
}
#[inline]
pub unsafe fn InvertRgn(hdc: HDC, hrgn: HRGN) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn InvertRgn(hdc : HDC, hrgn : HRGN) -> windows_core::BOOL);
    unsafe { InvertRgn(hdc, hrgn) }
}
#[inline]
pub unsafe fn IsRectEmpty(lprc: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn IsRectEmpty(lprc : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { IsRectEmpty(lprc) }
}
#[inline]
pub unsafe fn LPtoDP(hdc: HDC, lppt: &mut [super::super::Foundation::POINT]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn LPtoDP(hdc : HDC, lppt : *mut super::super::Foundation:: POINT, c : i32) -> windows_core::BOOL);
    unsafe { LPtoDP(hdc, core::mem::transmute(lppt.as_ptr()), lppt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn LineDDA(xstart: i32, ystart: i32, xend: i32, yend: i32, lpproc: LINEDDAPROC, data: Option<super::super::Foundation::LPARAM>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn LineDDA(xstart : i32, ystart : i32, xend : i32, yend : i32, lpproc : LINEDDAPROC, data : super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { LineDDA(xstart, ystart, xend, yend, lpproc, data.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LineTo(hdc: HDC, x: i32, y: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn LineTo(hdc : HDC, x : i32, y : i32) -> windows_core::BOOL);
    unsafe { LineTo(hdc, x, y) }
}
#[inline]
pub unsafe fn LoadBitmapA<P1>(hinstance: Option<super::super::Foundation::HINSTANCE>, lpbitmapname: P1) -> HBITMAP
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("user32.dll" "system" fn LoadBitmapA(hinstance : super::super::Foundation:: HINSTANCE, lpbitmapname : windows_core::PCSTR) -> HBITMAP);
    unsafe { LoadBitmapA(hinstance.unwrap_or(core::mem::zeroed()) as _, lpbitmapname.param().abi()) }
}
#[inline]
pub unsafe fn LoadBitmapW<P1>(hinstance: Option<super::super::Foundation::HINSTANCE>, lpbitmapname: P1) -> HBITMAP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("user32.dll" "system" fn LoadBitmapW(hinstance : super::super::Foundation:: HINSTANCE, lpbitmapname : windows_core::PCWSTR) -> HBITMAP);
    unsafe { LoadBitmapW(hinstance.unwrap_or(core::mem::zeroed()) as _, lpbitmapname.param().abi()) }
}
#[inline]
pub unsafe fn LockWindowUpdate(hwndlock: Option<super::super::Foundation::HWND>) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn LockWindowUpdate(hwndlock : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { LockWindowUpdate(hwndlock.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MapWindowPoints(hwndfrom: Option<super::super::Foundation::HWND>, hwndto: Option<super::super::Foundation::HWND>, lppoints: &mut [super::super::Foundation::POINT]) -> i32 {
    windows_link::link!("user32.dll" "system" fn MapWindowPoints(hwndfrom : super::super::Foundation:: HWND, hwndto : super::super::Foundation:: HWND, lppoints : *mut super::super::Foundation:: POINT, cpoints : u32) -> i32);
    unsafe { MapWindowPoints(hwndfrom.unwrap_or(core::mem::zeroed()) as _, hwndto.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lppoints.as_ptr()), lppoints.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn MaskBlt(hdcdest: HDC, xdest: i32, ydest: i32, width: i32, height: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32, rop: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn MaskBlt(hdcdest : HDC, xdest : i32, ydest : i32, width : i32, height : i32, hdcsrc : HDC, xsrc : i32, ysrc : i32, hbmmask : HBITMAP, xmask : i32, ymask : i32, rop : u32) -> windows_core::BOOL);
    unsafe { MaskBlt(hdcdest, xdest, ydest, width, height, hdcsrc, xsrc, ysrc, hbmmask, xmask, ymask, rop) }
}
#[inline]
pub unsafe fn MergeFontPackage(puchmergefontbuffer: *const u8, ulmergefontbuffersize: u32, puchfontpackagebuffer: *const u8, ulfontpackagebuffersize: u32, ppuchdestbuffer: *mut *mut u8, puldestbuffersize: *mut u32, pulbyteswritten: *mut u32, usmode: u16, lpfnallocate: CFP_ALLOCPROC, lpfnreallocate: CFP_REALLOCPROC, lpfnfree: CFP_FREEPROC, lpvreserved: *mut core::ffi::c_void) -> u32 {
    windows_link::link!("fontsub.dll" "C" fn MergeFontPackage(puchmergefontbuffer : *const u8, ulmergefontbuffersize : u32, puchfontpackagebuffer : *const u8, ulfontpackagebuffersize : u32, ppuchdestbuffer : *mut *mut u8, puldestbuffersize : *mut u32, pulbyteswritten : *mut u32, usmode : u16, lpfnallocate : CFP_ALLOCPROC, lpfnreallocate : CFP_REALLOCPROC, lpfnfree : CFP_FREEPROC, lpvreserved : *mut core::ffi::c_void) -> u32);
    unsafe { MergeFontPackage(puchmergefontbuffer, ulmergefontbuffersize, puchfontpackagebuffer, ulfontpackagebuffersize, ppuchdestbuffer as _, puldestbuffersize as _, pulbyteswritten as _, usmode, lpfnallocate, lpfnreallocate, lpfnfree, lpvreserved as _) }
}
#[inline]
pub unsafe fn ModifyWorldTransform(hdc: HDC, lpxf: Option<*const XFORM>, mode: MODIFY_WORLD_TRANSFORM_MODE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn ModifyWorldTransform(hdc : HDC, lpxf : *const XFORM, mode : MODIFY_WORLD_TRANSFORM_MODE) -> windows_core::BOOL);
    unsafe { ModifyWorldTransform(hdc, lpxf.unwrap_or(core::mem::zeroed()) as _, mode) }
}
#[inline]
pub unsafe fn MonitorFromPoint(pt: super::super::Foundation::POINT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR {
    windows_link::link!("user32.dll" "system" fn MonitorFromPoint(pt : super::super::Foundation:: POINT, dwflags : MONITOR_FROM_FLAGS) -> HMONITOR);
    unsafe { MonitorFromPoint(core::mem::transmute(pt), dwflags) }
}
#[inline]
pub unsafe fn MonitorFromRect(lprc: *const super::super::Foundation::RECT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR {
    windows_link::link!("user32.dll" "system" fn MonitorFromRect(lprc : *const super::super::Foundation:: RECT, dwflags : MONITOR_FROM_FLAGS) -> HMONITOR);
    unsafe { MonitorFromRect(lprc, dwflags) }
}
#[inline]
pub unsafe fn MonitorFromWindow(hwnd: super::super::Foundation::HWND, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR {
    windows_link::link!("user32.dll" "system" fn MonitorFromWindow(hwnd : super::super::Foundation:: HWND, dwflags : MONITOR_FROM_FLAGS) -> HMONITOR);
    unsafe { MonitorFromWindow(hwnd, dwflags) }
}
#[inline]
pub unsafe fn MoveToEx(hdc: HDC, x: i32, y: i32, lppt: Option<*mut super::super::Foundation::POINT>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn MoveToEx(hdc : HDC, x : i32, y : i32, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { MoveToEx(hdc, x, y, lppt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OffsetClipRgn(hdc: HDC, x: i32, y: i32) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn OffsetClipRgn(hdc : HDC, x : i32, y : i32) -> GDI_REGION_TYPE);
    unsafe { OffsetClipRgn(hdc, x, y) }
}
#[inline]
pub unsafe fn OffsetRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn OffsetRect(lprc : *mut super::super::Foundation:: RECT, dx : i32, dy : i32) -> windows_core::BOOL);
    unsafe { OffsetRect(lprc as _, dx, dy) }
}
#[inline]
pub unsafe fn OffsetRgn(hrgn: HRGN, x: i32, y: i32) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn OffsetRgn(hrgn : HRGN, x : i32, y : i32) -> GDI_REGION_TYPE);
    unsafe { OffsetRgn(hrgn, x, y) }
}
#[inline]
pub unsafe fn OffsetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: Option<*mut super::super::Foundation::POINT>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn OffsetViewportOrgEx(hdc : HDC, x : i32, y : i32, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { OffsetViewportOrgEx(hdc, x, y, lppt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OffsetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: Option<*mut super::super::Foundation::POINT>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn OffsetWindowOrgEx(hdc : HDC, x : i32, y : i32, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { OffsetWindowOrgEx(hdc, x, y, lppt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PaintDesktop(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn PaintDesktop(hdc : HDC) -> windows_core::BOOL);
    unsafe { PaintDesktop(hdc) }
}
#[inline]
pub unsafe fn PaintRgn(hdc: HDC, hrgn: HRGN) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PaintRgn(hdc : HDC, hrgn : HRGN) -> windows_core::BOOL);
    unsafe { PaintRgn(hdc, hrgn) }
}
#[inline]
pub unsafe fn PatBlt(hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: ROP_CODE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PatBlt(hdc : HDC, x : i32, y : i32, w : i32, h : i32, rop : ROP_CODE) -> windows_core::BOOL);
    unsafe { PatBlt(hdc, x, y, w, h, rop) }
}
#[inline]
pub unsafe fn PathToRegion(hdc: HDC) -> HRGN {
    windows_link::link!("gdi32.dll" "system" fn PathToRegion(hdc : HDC) -> HRGN);
    unsafe { PathToRegion(hdc) }
}
#[inline]
pub unsafe fn Pie(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn Pie(hdc : HDC, left : i32, top : i32, right : i32, bottom : i32, xr1 : i32, yr1 : i32, xr2 : i32, yr2 : i32) -> windows_core::BOOL);
    unsafe { Pie(hdc, left, top, right, bottom, xr1, yr1, xr2, yr2) }
}
#[inline]
pub unsafe fn PlayEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, lprect: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PlayEnhMetaFile(hdc : HDC, hmf : HENHMETAFILE, lprect : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { PlayEnhMetaFile(hdc, hmf, lprect) }
}
#[inline]
pub unsafe fn PlayEnhMetaFileRecord(hdc: HDC, pht: &[HANDLETABLE], pmr: *const ENHMETARECORD) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PlayEnhMetaFileRecord(hdc : HDC, pht : *const HANDLETABLE, pmr : *const ENHMETARECORD, cht : u32) -> windows_core::BOOL);
    unsafe { PlayEnhMetaFileRecord(hdc, core::mem::transmute(pht.as_ptr()), pmr, pht.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PlayMetaFile(hdc: HDC, hmf: HMETAFILE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PlayMetaFile(hdc : HDC, hmf : HMETAFILE) -> windows_core::BOOL);
    unsafe { PlayMetaFile(hdc, hmf) }
}
#[inline]
pub unsafe fn PlayMetaFileRecord(hdc: HDC, lphandletable: &[HANDLETABLE], lpmr: *const METARECORD) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PlayMetaFileRecord(hdc : HDC, lphandletable : *const HANDLETABLE, lpmr : *const METARECORD, noobjs : u32) -> windows_core::BOOL);
    unsafe { PlayMetaFileRecord(hdc, core::mem::transmute(lphandletable.as_ptr()), lpmr, lphandletable.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PlgBlt(hdcdest: HDC, lppoint: &[super::super::Foundation::POINT; 3], hdcsrc: HDC, xsrc: i32, ysrc: i32, width: i32, height: i32, hbmmask: Option<HBITMAP>, xmask: i32, ymask: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PlgBlt(hdcdest : HDC, lppoint : *const super::super::Foundation:: POINT, hdcsrc : HDC, xsrc : i32, ysrc : i32, width : i32, height : i32, hbmmask : HBITMAP, xmask : i32, ymask : i32) -> windows_core::BOOL);
    unsafe { PlgBlt(hdcdest, core::mem::transmute(lppoint.as_ptr()), hdcsrc, xsrc, ysrc, width, height, hbmmask.unwrap_or(core::mem::zeroed()) as _, xmask, ymask) }
}
#[inline]
pub unsafe fn PolyBezier(hdc: HDC, apt: &[super::super::Foundation::POINT]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PolyBezier(hdc : HDC, apt : *const super::super::Foundation:: POINT, cpt : u32) -> windows_core::BOOL);
    unsafe { PolyBezier(hdc, core::mem::transmute(apt.as_ptr()), apt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PolyBezierTo(hdc: HDC, apt: &[super::super::Foundation::POINT]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PolyBezierTo(hdc : HDC, apt : *const super::super::Foundation:: POINT, cpt : u32) -> windows_core::BOOL);
    unsafe { PolyBezierTo(hdc, core::mem::transmute(apt.as_ptr()), apt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PolyDraw(hdc: HDC, apt: *const super::super::Foundation::POINT, aj: *const u8, cpt: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PolyDraw(hdc : HDC, apt : *const super::super::Foundation:: POINT, aj : *const u8, cpt : i32) -> windows_core::BOOL);
    unsafe { PolyDraw(hdc, apt, aj, cpt) }
}
#[inline]
pub unsafe fn PolyPolygon(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: &[i32]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PolyPolygon(hdc : HDC, apt : *const super::super::Foundation:: POINT, asz : *const i32, csz : i32) -> windows_core::BOOL);
    unsafe { PolyPolygon(hdc, apt, core::mem::transmute(asz.as_ptr()), asz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PolyPolyline(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: &[u32]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PolyPolyline(hdc : HDC, apt : *const super::super::Foundation:: POINT, asz : *const u32, csz : u32) -> windows_core::BOOL);
    unsafe { PolyPolyline(hdc, apt, core::mem::transmute(asz.as_ptr()), asz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PolyTextOutA(hdc: HDC, ppt: &[POLYTEXTA]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PolyTextOutA(hdc : HDC, ppt : *const POLYTEXTA, nstrings : i32) -> windows_core::BOOL);
    unsafe { PolyTextOutA(hdc, core::mem::transmute(ppt.as_ptr()), ppt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PolyTextOutW(hdc: HDC, ppt: &[POLYTEXTW]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PolyTextOutW(hdc : HDC, ppt : *const POLYTEXTW, nstrings : i32) -> windows_core::BOOL);
    unsafe { PolyTextOutW(hdc, core::mem::transmute(ppt.as_ptr()), ppt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn Polygon(hdc: HDC, apt: &[super::super::Foundation::POINT]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn Polygon(hdc : HDC, apt : *const super::super::Foundation:: POINT, cpt : i32) -> windows_core::BOOL);
    unsafe { Polygon(hdc, core::mem::transmute(apt.as_ptr()), apt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn Polyline(hdc: HDC, apt: &[super::super::Foundation::POINT]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn Polyline(hdc : HDC, apt : *const super::super::Foundation:: POINT, cpt : i32) -> windows_core::BOOL);
    unsafe { Polyline(hdc, core::mem::transmute(apt.as_ptr()), apt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PolylineTo(hdc: HDC, apt: &[super::super::Foundation::POINT]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PolylineTo(hdc : HDC, apt : *const super::super::Foundation:: POINT, cpt : u32) -> windows_core::BOOL);
    unsafe { PolylineTo(hdc, core::mem::transmute(apt.as_ptr()), apt.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PtInRect(lprc: *const super::super::Foundation::RECT, pt: super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn PtInRect(lprc : *const super::super::Foundation:: RECT, pt : super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { PtInRect(lprc, core::mem::transmute(pt)) }
}
#[inline]
pub unsafe fn PtInRegion(hrgn: HRGN, x: i32, y: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PtInRegion(hrgn : HRGN, x : i32, y : i32) -> windows_core::BOOL);
    unsafe { PtInRegion(hrgn, x, y) }
}
#[inline]
pub unsafe fn PtVisible(hdc: HDC, x: i32, y: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PtVisible(hdc : HDC, x : i32, y : i32) -> windows_core::BOOL);
    unsafe { PtVisible(hdc, x, y) }
}
#[inline]
pub unsafe fn RealizePalette(hdc: HDC) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn RealizePalette(hdc : HDC) -> u32);
    unsafe { RealizePalette(hdc) }
}
#[inline]
pub unsafe fn RectInRegion(hrgn: HRGN, lprect: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn RectInRegion(hrgn : HRGN, lprect : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { RectInRegion(hrgn, lprect) }
}
#[inline]
pub unsafe fn RectVisible(hdc: HDC, lprect: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn RectVisible(hdc : HDC, lprect : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { RectVisible(hdc, lprect) }
}
#[inline]
pub unsafe fn Rectangle(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn Rectangle(hdc : HDC, left : i32, top : i32, right : i32, bottom : i32) -> windows_core::BOOL);
    unsafe { Rectangle(hdc, left, top, right, bottom) }
}
#[inline]
pub unsafe fn RedrawWindow(hwnd: Option<super::super::Foundation::HWND>, lprcupdate: Option<*const super::super::Foundation::RECT>, hrgnupdate: Option<HRGN>, flags: REDRAW_WINDOW_FLAGS) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn RedrawWindow(hwnd : super::super::Foundation:: HWND, lprcupdate : *const super::super::Foundation:: RECT, hrgnupdate : HRGN, flags : REDRAW_WINDOW_FLAGS) -> windows_core::BOOL);
    unsafe { RedrawWindow(hwnd.unwrap_or(core::mem::zeroed()) as _, lprcupdate.unwrap_or(core::mem::zeroed()) as _, hrgnupdate.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn ReleaseDC(hwnd: Option<super::super::Foundation::HWND>, hdc: HDC) -> i32 {
    windows_link::link!("user32.dll" "system" fn ReleaseDC(hwnd : super::super::Foundation:: HWND, hdc : HDC) -> i32);
    unsafe { ReleaseDC(hwnd.unwrap_or(core::mem::zeroed()) as _, hdc) }
}
#[inline]
pub unsafe fn RemoveFontMemResourceEx(h: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn RemoveFontMemResourceEx(h : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { RemoveFontMemResourceEx(h) }
}
#[inline]
pub unsafe fn RemoveFontResourceA<P0>(lpfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceA(lpfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { RemoveFontResourceA(lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn RemoveFontResourceExA<P0>(name: P0, fl: u32, pdv: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceExA(name : windows_core::PCSTR, fl : u32, pdv : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RemoveFontResourceExA(name.param().abi(), fl, pdv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RemoveFontResourceExW<P0>(name: P0, fl: u32, pdv: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceExW(name : windows_core::PCWSTR, fl : u32, pdv : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RemoveFontResourceExW(name.param().abi(), fl, pdv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RemoveFontResourceW<P0>(lpfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceW(lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RemoveFontResourceW(lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn ResetDCA(hdc: HDC, lpdm: *const DEVMODEA) -> HDC {
    windows_link::link!("gdi32.dll" "system" fn ResetDCA(hdc : HDC, lpdm : *const DEVMODEA) -> HDC);
    unsafe { ResetDCA(hdc, lpdm) }
}
#[inline]
pub unsafe fn ResetDCW(hdc: HDC, lpdm: *const DEVMODEW) -> HDC {
    windows_link::link!("gdi32.dll" "system" fn ResetDCW(hdc : HDC, lpdm : *const DEVMODEW) -> HDC);
    unsafe { ResetDCW(hdc, lpdm) }
}
#[inline]
pub unsafe fn ResizePalette(hpal: HPALETTE, n: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn ResizePalette(hpal : HPALETTE, n : u32) -> windows_core::BOOL);
    unsafe { ResizePalette(hpal, n) }
}
#[inline]
pub unsafe fn RestoreDC(hdc: HDC, nsaveddc: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn RestoreDC(hdc : HDC, nsaveddc : i32) -> windows_core::BOOL);
    unsafe { RestoreDC(hdc, nsaveddc) }
}
#[inline]
pub unsafe fn RoundRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn RoundRect(hdc : HDC, left : i32, top : i32, right : i32, bottom : i32, width : i32, height : i32) -> windows_core::BOOL);
    unsafe { RoundRect(hdc, left, top, right, bottom, width, height) }
}
#[inline]
pub unsafe fn SaveDC(hdc: HDC) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SaveDC(hdc : HDC) -> i32);
    unsafe { SaveDC(hdc) }
}
#[inline]
pub unsafe fn ScaleViewportExtEx(hdc: HDC, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: Option<*mut super::super::Foundation::SIZE>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn ScaleViewportExtEx(hdc : HDC, xn : i32, dx : i32, yn : i32, yd : i32, lpsz : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { ScaleViewportExtEx(hdc, xn, dx, yn, yd, lpsz.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ScaleWindowExtEx(hdc: HDC, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: Option<*mut super::super::Foundation::SIZE>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn ScaleWindowExtEx(hdc : HDC, xn : i32, xd : i32, yn : i32, yd : i32, lpsz : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { ScaleWindowExtEx(hdc, xn, xd, yn, yd, lpsz.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ScreenToClient(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn ScreenToClient(hwnd : super::super::Foundation:: HWND, lppoint : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { ScreenToClient(hwnd, lppoint as _) }
}
#[inline]
pub unsafe fn SelectClipPath(hdc: HDC, mode: RGN_COMBINE_MODE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SelectClipPath(hdc : HDC, mode : RGN_COMBINE_MODE) -> windows_core::BOOL);
    unsafe { SelectClipPath(hdc, mode) }
}
#[inline]
pub unsafe fn SelectClipRgn(hdc: HDC, hrgn: Option<HRGN>) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn SelectClipRgn(hdc : HDC, hrgn : HRGN) -> GDI_REGION_TYPE);
    unsafe { SelectClipRgn(hdc, hrgn.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ {
    windows_link::link!("gdi32.dll" "system" fn SelectObject(hdc : HDC, h : HGDIOBJ) -> HGDIOBJ);
    unsafe { SelectObject(hdc, h) }
}
#[inline]
pub unsafe fn SelectPalette(hdc: HDC, hpal: HPALETTE, bforcebkgd: bool) -> HPALETTE {
    windows_link::link!("gdi32.dll" "system" fn SelectPalette(hdc : HDC, hpal : HPALETTE, bforcebkgd : windows_core::BOOL) -> HPALETTE);
    unsafe { SelectPalette(hdc, hpal, bforcebkgd.into()) }
}
#[inline]
pub unsafe fn SetArcDirection(hdc: HDC, dir: ARC_DIRECTION) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetArcDirection(hdc : HDC, dir : ARC_DIRECTION) -> i32);
    unsafe { SetArcDirection(hdc, dir) }
}
#[inline]
pub unsafe fn SetBitmapBits(hbm: HBITMAP, cb: u32, pvbits: *const core::ffi::c_void) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetBitmapBits(hbm : HBITMAP, cb : u32, pvbits : *const core::ffi::c_void) -> i32);
    unsafe { SetBitmapBits(hbm, cb, pvbits) }
}
#[inline]
pub unsafe fn SetBitmapDimensionEx(hbm: HBITMAP, w: i32, h: i32, lpsz: Option<*mut super::super::Foundation::SIZE>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetBitmapDimensionEx(hbm : HBITMAP, w : i32, h : i32, lpsz : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { SetBitmapDimensionEx(hbm, w, h, lpsz.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetBkColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn SetBkColor(hdc : HDC, color : super::super::Foundation:: COLORREF) -> super::super::Foundation:: COLORREF);
    unsafe { SetBkColor(hdc, color) }
}
#[inline]
pub unsafe fn SetBkMode(hdc: HDC, mode: BACKGROUND_MODE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetBkMode(hdc : HDC, mode : i32) -> i32);
    unsafe { SetBkMode(hdc, mode.0 as _) }
}
#[inline]
pub unsafe fn SetBoundsRect(hdc: HDC, lprect: Option<*const super::super::Foundation::RECT>, flags: SET_BOUNDS_RECT_FLAGS) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn SetBoundsRect(hdc : HDC, lprect : *const super::super::Foundation:: RECT, flags : SET_BOUNDS_RECT_FLAGS) -> u32);
    unsafe { SetBoundsRect(hdc, lprect.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetBrushOrgEx(hdc: HDC, x: i32, y: i32, lppt: Option<*mut super::super::Foundation::POINT>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetBrushOrgEx(hdc : HDC, x : i32, y : i32, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { SetBrushOrgEx(hdc, x, y, lppt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetColorAdjustment(hdc: HDC, lpca: *const COLORADJUSTMENT) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetColorAdjustment(hdc : HDC, lpca : *const COLORADJUSTMENT) -> windows_core::BOOL);
    unsafe { SetColorAdjustment(hdc, lpca) }
}
#[inline]
pub unsafe fn SetDCBrushColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn SetDCBrushColor(hdc : HDC, color : super::super::Foundation:: COLORREF) -> super::super::Foundation:: COLORREF);
    unsafe { SetDCBrushColor(hdc, color) }
}
#[inline]
pub unsafe fn SetDCPenColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn SetDCPenColor(hdc : HDC, color : super::super::Foundation:: COLORREF) -> super::super::Foundation:: COLORREF);
    unsafe { SetDCPenColor(hdc, color) }
}
#[inline]
pub unsafe fn SetDIBColorTable(hdc: HDC, istart: u32, prgbq: &[RGBQUAD]) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn SetDIBColorTable(hdc : HDC, istart : u32, centries : u32, prgbq : *const RGBQUAD) -> u32);
    unsafe { SetDIBColorTable(hdc, istart, prgbq.len().try_into().unwrap(), core::mem::transmute(prgbq.as_ptr())) }
}
#[inline]
pub unsafe fn SetDIBits(hdc: Option<HDC>, hbm: HBITMAP, start: u32, clines: u32, lpbits: *const core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetDIBits(hdc : HDC, hbm : HBITMAP, start : u32, clines : u32, lpbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, coloruse : DIB_USAGE) -> i32);
    unsafe { SetDIBits(hdc.unwrap_or(core::mem::zeroed()) as _, hbm, start, clines, lpbits, lpbmi, coloruse) }
}
#[inline]
pub unsafe fn SetDIBitsToDevice(hdc: HDC, xdest: i32, ydest: i32, w: u32, h: u32, xsrc: i32, ysrc: i32, startscan: u32, clines: u32, lpvbits: *const core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetDIBitsToDevice(hdc : HDC, xdest : i32, ydest : i32, w : u32, h : u32, xsrc : i32, ysrc : i32, startscan : u32, clines : u32, lpvbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, coloruse : DIB_USAGE) -> i32);
    unsafe { SetDIBitsToDevice(hdc, xdest, ydest, w, h, xsrc, ysrc, startscan, clines, lpvbits, lpbmi, coloruse) }
}
#[inline]
pub unsafe fn SetEnhMetaFileBits(pb: &[u8]) -> HENHMETAFILE {
    windows_link::link!("gdi32.dll" "system" fn SetEnhMetaFileBits(nsize : u32, pb : *const u8) -> HENHMETAFILE);
    unsafe { SetEnhMetaFileBits(pb.len().try_into().unwrap(), core::mem::transmute(pb.as_ptr())) }
}
#[inline]
pub unsafe fn SetGraphicsMode(hdc: HDC, imode: GRAPHICS_MODE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetGraphicsMode(hdc : HDC, imode : GRAPHICS_MODE) -> i32);
    unsafe { SetGraphicsMode(hdc, imode) }
}
#[inline]
pub unsafe fn SetLayout(hdc: HDC, l: DC_LAYOUT) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn SetLayout(hdc : HDC, l : DC_LAYOUT) -> u32);
    unsafe { SetLayout(hdc, l) }
}
#[inline]
pub unsafe fn SetMapMode(hdc: HDC, imode: HDC_MAP_MODE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetMapMode(hdc : HDC, imode : HDC_MAP_MODE) -> i32);
    unsafe { SetMapMode(hdc, imode) }
}
#[inline]
pub unsafe fn SetMapperFlags(hdc: HDC, flags: u32) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn SetMapperFlags(hdc : HDC, flags : u32) -> u32);
    unsafe { SetMapperFlags(hdc, flags) }
}
#[inline]
pub unsafe fn SetMetaFileBitsEx(lpdata: &[u8]) -> HMETAFILE {
    windows_link::link!("gdi32.dll" "system" fn SetMetaFileBitsEx(cbbuffer : u32, lpdata : *const u8) -> HMETAFILE);
    unsafe { SetMetaFileBitsEx(lpdata.len().try_into().unwrap(), core::mem::transmute(lpdata.as_ptr())) }
}
#[inline]
pub unsafe fn SetMetaRgn(hdc: HDC) -> GDI_REGION_TYPE {
    windows_link::link!("gdi32.dll" "system" fn SetMetaRgn(hdc : HDC) -> GDI_REGION_TYPE);
    unsafe { SetMetaRgn(hdc) }
}
#[inline]
pub unsafe fn SetMiterLimit(hdc: HDC, limit: f32, old: Option<*mut f32>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetMiterLimit(hdc : HDC, limit : f32, old : *mut f32) -> windows_core::BOOL);
    unsafe { SetMiterLimit(hdc, limit, old.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetPaletteEntries(hpal: HPALETTE, istart: u32, ppalentries: &[PALETTEENTRY]) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn SetPaletteEntries(hpal : HPALETTE, istart : u32, centries : u32, ppalentries : *const PALETTEENTRY) -> u32);
    unsafe { SetPaletteEntries(hpal, istart, ppalentries.len().try_into().unwrap(), core::mem::transmute(ppalentries.as_ptr())) }
}
#[inline]
pub unsafe fn SetPixel(hdc: HDC, x: i32, y: i32, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn SetPixel(hdc : HDC, x : i32, y : i32, color : super::super::Foundation:: COLORREF) -> super::super::Foundation:: COLORREF);
    unsafe { SetPixel(hdc, x, y, color) }
}
#[inline]
pub unsafe fn SetPixelV(hdc: HDC, x: i32, y: i32, color: super::super::Foundation::COLORREF) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetPixelV(hdc : HDC, x : i32, y : i32, color : super::super::Foundation:: COLORREF) -> windows_core::BOOL);
    unsafe { SetPixelV(hdc, x, y, color) }
}
#[inline]
pub unsafe fn SetPolyFillMode(hdc: HDC, mode: CREATE_POLYGON_RGN_MODE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetPolyFillMode(hdc : HDC, mode : CREATE_POLYGON_RGN_MODE) -> i32);
    unsafe { SetPolyFillMode(hdc, mode) }
}
#[inline]
pub unsafe fn SetROP2(hdc: HDC, rop2: R2_MODE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetROP2(hdc : HDC, rop2 : R2_MODE) -> i32);
    unsafe { SetROP2(hdc, rop2) }
}
#[inline]
pub unsafe fn SetRect(lprc: *mut super::super::Foundation::RECT, xleft: i32, ytop: i32, xright: i32, ybottom: i32) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn SetRect(lprc : *mut super::super::Foundation:: RECT, xleft : i32, ytop : i32, xright : i32, ybottom : i32) -> windows_core::BOOL);
    unsafe { SetRect(lprc as _, xleft, ytop, xright, ybottom) }
}
#[inline]
pub unsafe fn SetRectEmpty(lprc: *mut super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn SetRectEmpty(lprc : *mut super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { SetRectEmpty(lprc as _) }
}
#[inline]
pub unsafe fn SetRectRgn(hrgn: HRGN, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetRectRgn(hrgn : HRGN, left : i32, top : i32, right : i32, bottom : i32) -> windows_core::BOOL);
    unsafe { SetRectRgn(hrgn, left, top, right, bottom) }
}
#[inline]
pub unsafe fn SetStretchBltMode(hdc: HDC, mode: STRETCH_BLT_MODE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetStretchBltMode(hdc : HDC, mode : STRETCH_BLT_MODE) -> i32);
    unsafe { SetStretchBltMode(hdc, mode) }
}
#[inline]
pub unsafe fn SetSysColors(celements: i32, lpaelements: *const i32, lpargbvalues: *const super::super::Foundation::COLORREF) -> windows_core::Result<()> {
    windows_link::link!("user32.dll" "system" fn SetSysColors(celements : i32, lpaelements : *const i32, lpargbvalues : *const super::super::Foundation:: COLORREF) -> windows_core::BOOL);
    unsafe { SetSysColors(celements, lpaelements, lpargbvalues).ok() }
}
#[inline]
pub unsafe fn SetSystemPaletteUse(hdc: HDC, r#use: SYSTEM_PALETTE_USE) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn SetSystemPaletteUse(hdc : HDC, r#use : SYSTEM_PALETTE_USE) -> u32);
    unsafe { SetSystemPaletteUse(hdc, r#use) }
}
#[inline]
pub unsafe fn SetTextAlign(hdc: HDC, align: TEXT_ALIGN_OPTIONS) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn SetTextAlign(hdc : HDC, align : TEXT_ALIGN_OPTIONS) -> u32);
    unsafe { SetTextAlign(hdc, align) }
}
#[inline]
pub unsafe fn SetTextCharacterExtra(hdc: HDC, extra: i32) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn SetTextCharacterExtra(hdc : HDC, extra : i32) -> i32);
    unsafe { SetTextCharacterExtra(hdc, extra) }
}
#[inline]
pub unsafe fn SetTextColor(hdc: HDC, color: super::super::Foundation::COLORREF) -> super::super::Foundation::COLORREF {
    windows_link::link!("gdi32.dll" "system" fn SetTextColor(hdc : HDC, color : super::super::Foundation:: COLORREF) -> super::super::Foundation:: COLORREF);
    unsafe { SetTextColor(hdc, color) }
}
#[inline]
pub unsafe fn SetTextJustification(hdc: HDC, extra: i32, count: i32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetTextJustification(hdc : HDC, extra : i32, count : i32) -> windows_core::BOOL);
    unsafe { SetTextJustification(hdc, extra, count) }
}
#[inline]
pub unsafe fn SetViewportExtEx(hdc: HDC, x: i32, y: i32, lpsz: Option<*mut super::super::Foundation::SIZE>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetViewportExtEx(hdc : HDC, x : i32, y : i32, lpsz : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { SetViewportExtEx(hdc, x, y, lpsz.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: Option<*mut super::super::Foundation::POINT>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetViewportOrgEx(hdc : HDC, x : i32, y : i32, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { SetViewportOrgEx(hdc, x, y, lppt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetWindowExtEx(hdc: HDC, x: i32, y: i32, lpsz: Option<*mut super::super::Foundation::SIZE>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetWindowExtEx(hdc : HDC, x : i32, y : i32, lpsz : *mut super::super::Foundation:: SIZE) -> windows_core::BOOL);
    unsafe { SetWindowExtEx(hdc, x, y, lpsz.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: Option<*mut super::super::Foundation::POINT>) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetWindowOrgEx(hdc : HDC, x : i32, y : i32, lppt : *mut super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { SetWindowOrgEx(hdc, x, y, lppt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: Option<HRGN>, bredraw: bool) -> i32 {
    windows_link::link!("user32.dll" "system" fn SetWindowRgn(hwnd : super::super::Foundation:: HWND, hrgn : HRGN, bredraw : windows_core::BOOL) -> i32);
    unsafe { SetWindowRgn(hwnd, hrgn.unwrap_or(core::mem::zeroed()) as _, bredraw.into()) }
}
#[inline]
pub unsafe fn SetWorldTransform(hdc: HDC, lpxf: *const XFORM) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn SetWorldTransform(hdc : HDC, lpxf : *const XFORM) -> windows_core::BOOL);
    unsafe { SetWorldTransform(hdc, lpxf) }
}
#[inline]
pub unsafe fn StretchBlt(hdcdest: HDC, xdest: i32, ydest: i32, wdest: i32, hdest: i32, hdcsrc: Option<HDC>, xsrc: i32, ysrc: i32, wsrc: i32, hsrc: i32, rop: ROP_CODE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn StretchBlt(hdcdest : HDC, xdest : i32, ydest : i32, wdest : i32, hdest : i32, hdcsrc : HDC, xsrc : i32, ysrc : i32, wsrc : i32, hsrc : i32, rop : ROP_CODE) -> windows_core::BOOL);
    unsafe { StretchBlt(hdcdest, xdest, ydest, wdest, hdest, hdcsrc.unwrap_or(core::mem::zeroed()) as _, xsrc, ysrc, wsrc, hsrc, rop) }
}
#[inline]
pub unsafe fn StretchDIBits(hdc: HDC, xdest: i32, ydest: i32, destwidth: i32, destheight: i32, xsrc: i32, ysrc: i32, srcwidth: i32, srcheight: i32, lpbits: Option<*const core::ffi::c_void>, lpbmi: *const BITMAPINFO, iusage: DIB_USAGE, rop: ROP_CODE) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn StretchDIBits(hdc : HDC, xdest : i32, ydest : i32, destwidth : i32, destheight : i32, xsrc : i32, ysrc : i32, srcwidth : i32, srcheight : i32, lpbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, iusage : DIB_USAGE, rop : ROP_CODE) -> i32);
    unsafe { StretchDIBits(hdc, xdest, ydest, destwidth, destheight, xsrc, ysrc, srcwidth, srcheight, lpbits.unwrap_or(core::mem::zeroed()) as _, lpbmi, iusage, rop) }
}
#[inline]
pub unsafe fn StrokeAndFillPath(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn StrokeAndFillPath(hdc : HDC) -> windows_core::BOOL);
    unsafe { StrokeAndFillPath(hdc) }
}
#[inline]
pub unsafe fn StrokePath(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn StrokePath(hdc : HDC) -> windows_core::BOOL);
    unsafe { StrokePath(hdc) }
}
#[inline]
pub unsafe fn SubtractRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn SubtractRect(lprcdst : *mut super::super::Foundation:: RECT, lprcsrc1 : *const super::super::Foundation:: RECT, lprcsrc2 : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { SubtractRect(lprcdst as _, lprcsrc1, lprcsrc2) }
}
#[inline]
pub unsafe fn TTCharToUnicode(hdc: HDC, puccharcodes: &[u8], pusshortcodes: &mut [u16], ulflags: u32) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTCharToUnicode(hdc : HDC, puccharcodes : *const u8, ulcharcodesize : u32, pusshortcodes : *mut u16, ulshortcodesize : u32, ulflags : u32) -> i32);
    unsafe { TTCharToUnicode(hdc, core::mem::transmute(puccharcodes.as_ptr()), puccharcodes.len().try_into().unwrap(), core::mem::transmute(pusshortcodes.as_ptr()), pusshortcodes.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn TTDeleteEmbeddedFont(hfontreference: super::super::Foundation::HANDLE, ulflags: u32, pulstatus: *mut u32) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTDeleteEmbeddedFont(hfontreference : super::super::Foundation:: HANDLE, ulflags : u32, pulstatus : *mut u32) -> i32);
    unsafe { TTDeleteEmbeddedFont(hfontreference, ulflags, pulstatus as _) }
}
#[inline]
pub unsafe fn TTEmbedFont(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const core::ffi::c_void, puscharcodeset: &[u16], uslanguage: u16, pttembedinfo: Option<*const TTEMBEDINFO>) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTEmbedFont(hdc : HDC, ulflags : TTEMBED_FLAGS, ulcharset : EMBED_FONT_CHARSET, pulprivstatus : *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, puscharcodeset : *const u16, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
    unsafe { TTEmbedFont(hdc, ulflags, ulcharset, pulprivstatus as _, pulstatus as _, lpfnwritetostream, lpvwritestream, core::mem::transmute(puscharcodeset.as_ptr()), puscharcodeset.len().try_into().unwrap(), uslanguage, pttembedinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TTEmbedFontEx(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const core::ffi::c_void, pulcharcodeset: &[u32], uslanguage: u16, pttembedinfo: Option<*const TTEMBEDINFO>) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTEmbedFontEx(hdc : HDC, ulflags : TTEMBED_FLAGS, ulcharset : EMBED_FONT_CHARSET, pulprivstatus : *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, pulcharcodeset : *const u32, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
    unsafe { TTEmbedFontEx(hdc, ulflags, ulcharset, pulprivstatus as _, pulstatus as _, lpfnwritetostream, lpvwritestream, core::mem::transmute(pulcharcodeset.as_ptr()), pulcharcodeset.len().try_into().unwrap(), uslanguage, pttembedinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TTEmbedFontFromFileA<P1>(hdc: HDC, szfontfilename: P1, usttcindex: u16, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const core::ffi::c_void, puscharcodeset: &[u16], uslanguage: u16, pttembedinfo: Option<*const TTEMBEDINFO>) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("t2embed.dll" "system" fn TTEmbedFontFromFileA(hdc : HDC, szfontfilename : windows_core::PCSTR, usttcindex : u16, ulflags : TTEMBED_FLAGS, ulcharset : EMBED_FONT_CHARSET, pulprivstatus : *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, puscharcodeset : *const u16, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
    unsafe { TTEmbedFontFromFileA(hdc, szfontfilename.param().abi(), usttcindex, ulflags, ulcharset, pulprivstatus as _, pulstatus as _, lpfnwritetostream, lpvwritestream, core::mem::transmute(puscharcodeset.as_ptr()), puscharcodeset.len().try_into().unwrap(), uslanguage, pttembedinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TTEnableEmbeddingForFacename<P0>(lpszfacename: P0, benable: bool) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("t2embed.dll" "system" fn TTEnableEmbeddingForFacename(lpszfacename : windows_core::PCSTR, benable : windows_core::BOOL) -> i32);
    unsafe { TTEnableEmbeddingForFacename(lpszfacename.param().abi(), benable.into()) }
}
#[inline]
pub unsafe fn TTGetEmbeddedFontInfo(ulflags: TTEMBED_FLAGS, pulprivstatus: *mut u32, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut u32, lpfnreadfromstream: READEMBEDPROC, lpvreadstream: *const core::ffi::c_void, pttloadinfo: Option<*const TTLOADINFO>) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTGetEmbeddedFontInfo(ulflags : TTEMBED_FLAGS, pulprivstatus : *mut u32, ulprivs : FONT_LICENSE_PRIVS, pulstatus : *mut u32, lpfnreadfromstream : READEMBEDPROC, lpvreadstream : *const core::ffi::c_void, pttloadinfo : *const TTLOADINFO) -> i32);
    unsafe { TTGetEmbeddedFontInfo(ulflags, pulprivstatus as _, ulprivs, pulstatus as _, lpfnreadfromstream, lpvreadstream, pttloadinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TTGetEmbeddingType(hdc: HDC, pulembedtype: *mut EMBEDDED_FONT_PRIV_STATUS) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTGetEmbeddingType(hdc : HDC, pulembedtype : *mut EMBEDDED_FONT_PRIV_STATUS) -> i32);
    unsafe { TTGetEmbeddingType(hdc, pulembedtype as _) }
}
#[inline]
pub unsafe fn TTGetNewFontName(phfontreference: *const super::super::Foundation::HANDLE, wzwinfamilyname: &mut [u16], szmacfamilyname: &mut [u8]) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTGetNewFontName(phfontreference : *const super::super::Foundation:: HANDLE, wzwinfamilyname : windows_core::PWSTR, cchmaxwinname : i32, szmacfamilyname : windows_core::PSTR, cchmaxmacname : i32) -> i32);
    unsafe { TTGetNewFontName(phfontreference, core::mem::transmute(wzwinfamilyname.as_ptr()), wzwinfamilyname.len().try_into().unwrap(), core::mem::transmute(szmacfamilyname.as_ptr()), szmacfamilyname.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn TTIsEmbeddingEnabled(hdc: HDC, pbenabled: *mut windows_core::BOOL) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTIsEmbeddingEnabled(hdc : HDC, pbenabled : *mut windows_core::BOOL) -> i32);
    unsafe { TTIsEmbeddingEnabled(hdc, pbenabled as _) }
}
#[inline]
pub unsafe fn TTIsEmbeddingEnabledForFacename<P0>(lpszfacename: P0, pbenabled: *mut windows_core::BOOL) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("t2embed.dll" "system" fn TTIsEmbeddingEnabledForFacename(lpszfacename : windows_core::PCSTR, pbenabled : *mut windows_core::BOOL) -> i32);
    unsafe { TTIsEmbeddingEnabledForFacename(lpszfacename.param().abi(), pbenabled as _) }
}
#[inline]
pub unsafe fn TTLoadEmbeddedFont<P7, P8>(phfontreference: *mut super::super::Foundation::HANDLE, ulflags: u32, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut TTLOAD_EMBEDDED_FONT_STATUS, lpfnreadfromstream: READEMBEDPROC, lpvreadstream: *const core::ffi::c_void, szwinfamilyname: P7, szmacfamilyname: P8, pttloadinfo: Option<*const TTLOADINFO>) -> i32
where
    P7: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCSTR>,
{
    windows_link::link!("t2embed.dll" "system" fn TTLoadEmbeddedFont(phfontreference : *mut super::super::Foundation:: HANDLE, ulflags : u32, pulprivstatus : *mut EMBEDDED_FONT_PRIV_STATUS, ulprivs : FONT_LICENSE_PRIVS, pulstatus : *mut TTLOAD_EMBEDDED_FONT_STATUS, lpfnreadfromstream : READEMBEDPROC, lpvreadstream : *const core::ffi::c_void, szwinfamilyname : windows_core::PCWSTR, szmacfamilyname : windows_core::PCSTR, pttloadinfo : *const TTLOADINFO) -> i32);
    unsafe { TTLoadEmbeddedFont(phfontreference as _, ulflags, pulprivstatus as _, ulprivs, pulstatus as _, lpfnreadfromstream, lpvreadstream, szwinfamilyname.param().abi(), szmacfamilyname.param().abi(), pttloadinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TTRunValidationTests(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMS) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTRunValidationTests(hdc : HDC, ptestparam : *const TTVALIDATIONTESTSPARAMS) -> i32);
    unsafe { TTRunValidationTests(hdc, ptestparam) }
}
#[inline]
pub unsafe fn TTRunValidationTestsEx(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMSEX) -> i32 {
    windows_link::link!("t2embed.dll" "system" fn TTRunValidationTestsEx(hdc : HDC, ptestparam : *const TTVALIDATIONTESTSPARAMSEX) -> i32);
    unsafe { TTRunValidationTestsEx(hdc, ptestparam) }
}
#[inline]
pub unsafe fn TabbedTextOutA(hdc: HDC, x: i32, y: i32, lpstring: &[u8], lpntabstoppositions: Option<&[i32]>, ntaborigin: i32) -> i32 {
    windows_link::link!("user32.dll" "system" fn TabbedTextOutA(hdc : HDC, x : i32, y : i32, lpstring : windows_core::PCSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32, ntaborigin : i32) -> i32);
    unsafe { TabbedTextOutA(hdc, x, y, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpntabstoppositions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpntabstoppositions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ntaborigin) }
}
#[inline]
pub unsafe fn TabbedTextOutW(hdc: HDC, x: i32, y: i32, lpstring: &[u16], lpntabstoppositions: Option<&[i32]>, ntaborigin: i32) -> i32 {
    windows_link::link!("user32.dll" "system" fn TabbedTextOutW(hdc : HDC, x : i32, y : i32, lpstring : windows_core::PCWSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32, ntaborigin : i32) -> i32);
    unsafe { TabbedTextOutW(hdc, x, y, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpntabstoppositions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpntabstoppositions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ntaborigin) }
}
#[inline]
pub unsafe fn TextOutA(hdc: HDC, x: i32, y: i32, lpstring: &[u8]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn TextOutA(hdc : HDC, x : i32, y : i32, lpstring : windows_core::PCSTR, c : i32) -> windows_core::BOOL);
    unsafe { TextOutA(hdc, x, y, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn TextOutW(hdc: HDC, x: i32, y: i32, lpstring: &[u16]) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn TextOutW(hdc : HDC, x : i32, y : i32, lpstring : windows_core::PCWSTR, c : i32) -> windows_core::BOOL);
    unsafe { TextOutW(hdc, x, y, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn TransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> windows_core::BOOL {
    windows_link::link!("msimg32.dll" "system" fn TransparentBlt(hdcdest : HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, crtransparent : u32) -> windows_core::BOOL);
    unsafe { TransparentBlt(hdcdest, xorigindest, yorigindest, wdest, hdest, hdcsrc, xoriginsrc, yoriginsrc, wsrc, hsrc, crtransparent) }
}
#[inline]
pub unsafe fn UnionRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn UnionRect(lprcdst : *mut super::super::Foundation:: RECT, lprcsrc1 : *const super::super::Foundation:: RECT, lprcsrc2 : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { UnionRect(lprcdst as _, lprcsrc1, lprcsrc2) }
}
#[inline]
pub unsafe fn UnrealizeObject(h: HGDIOBJ) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn UnrealizeObject(h : HGDIOBJ) -> windows_core::BOOL);
    unsafe { UnrealizeObject(h) }
}
#[inline]
pub unsafe fn UpdateColors(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn UpdateColors(hdc : HDC) -> windows_core::BOOL);
    unsafe { UpdateColors(hdc) }
}
#[inline]
pub unsafe fn UpdateWindow(hwnd: super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn UpdateWindow(hwnd : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { UpdateWindow(hwnd) }
}
#[inline]
pub unsafe fn ValidateRect(hwnd: Option<super::super::Foundation::HWND>, lprect: Option<*const super::super::Foundation::RECT>) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn ValidateRect(hwnd : super::super::Foundation:: HWND, lprect : *const super::super::Foundation:: RECT) -> windows_core::BOOL);
    unsafe { ValidateRect(hwnd.unwrap_or(core::mem::zeroed()) as _, lprect.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ValidateRgn(hwnd: super::super::Foundation::HWND, hrgn: Option<HRGN>) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn ValidateRgn(hwnd : super::super::Foundation:: HWND, hrgn : HRGN) -> windows_core::BOOL);
    unsafe { ValidateRgn(hwnd, hrgn.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WidenPath(hdc: HDC) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn WidenPath(hdc : HDC) -> windows_core::BOOL);
    unsafe { WidenPath(hdc) }
}
#[inline]
pub unsafe fn WindowFromDC(hdc: HDC) -> super::super::Foundation::HWND {
    windows_link::link!("user32.dll" "system" fn WindowFromDC(hdc : HDC) -> super::super::Foundation:: HWND);
    unsafe { WindowFromDC(hdc) }
}
#[inline]
pub unsafe fn wglSwapMultipleBuffers(param0: u32, param1: *const WGLSWAP) -> u32 {
    windows_link::link!("opengl32.dll" "system" fn wglSwapMultipleBuffers(param0 : u32, param1 : *const WGLSWAP) -> u32);
    unsafe { wglSwapMultipleBuffers(param0, param1) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ABC {
    pub abcA: i32,
    pub abcB: u32,
    pub abcC: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ABCFLOAT {
    pub abcfA: f32,
    pub abcfB: f32,
    pub abcfC: f32,
}
pub const ABORTDOC: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ABORTPATH {
    pub emr: EMR,
}
pub const ABSOLUTE: u32 = 1u32;
pub const AC_SRC_ALPHA: u32 = 1u32;
pub const AC_SRC_OVER: u32 = 0u32;
pub const AD_CLOCKWISE: ARC_DIRECTION = ARC_DIRECTION(2i32);
pub const AD_COUNTERCLOCKWISE: ARC_DIRECTION = ARC_DIRECTION(1i32);
pub const ALTERNATE: CREATE_POLYGON_RGN_MODE = CREATE_POLYGON_RGN_MODE(1i32);
pub const ANSI_CHARSET: FONT_CHARSET = FONT_CHARSET(0u8);
pub const ANSI_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(11i32);
pub const ANSI_VAR_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(12i32);
pub const ANTIALIASED_QUALITY: FONT_QUALITY = FONT_QUALITY(4u8);
pub const ARABIC_CHARSET: FONT_CHARSET = FONT_CHARSET(178u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ARC_DIRECTION(pub i32);
pub const ASPECTX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(40u32);
pub const ASPECTXY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(44u32);
pub const ASPECTY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(42u32);
pub const ASPECT_FILTERING: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AXESLISTA {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOA; 16],
}
impl Default for AXESLISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AXESLISTW {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOW; 16],
}
impl Default for AXESLISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AXISINFOA {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u8; 16],
}
impl Default for AXISINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AXISINFOW {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u16; 16],
}
impl Default for AXISINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BACKGROUND_MODE(pub u32);
pub const BALTIC_CHARSET: FONT_CHARSET = FONT_CHARSET(186u8);
pub const BANDINFO: u32 = 24u32;
pub const BDR_INNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(12u32);
pub const BDR_OUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(3u32);
pub const BDR_RAISED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(5u32);
pub const BDR_RAISEDINNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(4u32);
pub const BDR_RAISEDOUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(1u32);
pub const BDR_SUNKEN: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(10u32);
pub const BDR_SUNKENINNER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(8u32);
pub const BDR_SUNKENOUTER: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(2u32);
pub const BEGIN_PATH: u32 = 4096u32;
pub const BF_ADJUST: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(8192u32);
pub const BF_BOTTOM: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(8u32);
pub const BF_BOTTOMLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(9u32);
pub const BF_BOTTOMRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(12u32);
pub const BF_DIAGONAL: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(16u32);
pub const BF_DIAGONAL_ENDBOTTOMLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(25u32);
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(28u32);
pub const BF_DIAGONAL_ENDTOPLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(19u32);
pub const BF_DIAGONAL_ENDTOPRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(22u32);
pub const BF_FLAT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(16384u32);
pub const BF_LEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(1u32);
pub const BF_MIDDLE: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(2048u32);
pub const BF_MONO: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(32768u32);
pub const BF_RECT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(15u32);
pub const BF_RIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(4u32);
pub const BF_SOFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(4096u32);
pub const BF_TOP: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(2u32);
pub const BF_TOPLEFT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(3u32);
pub const BF_TOPRIGHT: DRAW_EDGE_FLAGS = DRAW_EDGE_FLAGS(6u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BITMAP {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub bmBits: *mut core::ffi::c_void,
}
impl Default for BITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BITMAPCOREHEADER {
    pub bcSize: u32,
    pub bcWidth: u16,
    pub bcHeight: u16,
    pub bcPlanes: u16,
    pub bcBitCount: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BITMAPCOREINFO {
    pub bmciHeader: BITMAPCOREHEADER,
    pub bmciColors: [RGBTRIPLE; 1],
}
impl Default for BITMAPCOREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct BITMAPFILEHEADER {
    pub bfType: u16,
    pub bfSize: u32,
    pub bfReserved1: u16,
    pub bfReserved2: u16,
    pub bfOffBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}
impl Default for BITMAPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BITMAPV4HEADER {
    pub bV4Size: u32,
    pub bV4Width: i32,
    pub bV4Height: i32,
    pub bV4Planes: u16,
    pub bV4BitCount: u16,
    pub bV4V4Compression: BI_COMPRESSION,
    pub bV4SizeImage: u32,
    pub bV4XPelsPerMeter: i32,
    pub bV4YPelsPerMeter: i32,
    pub bV4ClrUsed: u32,
    pub bV4ClrImportant: u32,
    pub bV4RedMask: u32,
    pub bV4GreenMask: u32,
    pub bV4BlueMask: u32,
    pub bV4AlphaMask: u32,
    pub bV4CSType: u32,
    pub bV4Endpoints: CIEXYZTRIPLE,
    pub bV4GammaRed: u32,
    pub bV4GammaGreen: u32,
    pub bV4GammaBlue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BITMAPV5HEADER {
    pub bV5Size: u32,
    pub bV5Width: i32,
    pub bV5Height: i32,
    pub bV5Planes: u16,
    pub bV5BitCount: u16,
    pub bV5Compression: BI_COMPRESSION,
    pub bV5SizeImage: u32,
    pub bV5XPelsPerMeter: i32,
    pub bV5YPelsPerMeter: i32,
    pub bV5ClrUsed: u32,
    pub bV5ClrImportant: u32,
    pub bV5RedMask: u32,
    pub bV5GreenMask: u32,
    pub bV5BlueMask: u32,
    pub bV5AlphaMask: u32,
    pub bV5CSType: u32,
    pub bV5Endpoints: CIEXYZTRIPLE,
    pub bV5GammaRed: u32,
    pub bV5GammaGreen: u32,
    pub bV5GammaBlue: u32,
    pub bV5Intent: u32,
    pub bV5ProfileData: u32,
    pub bV5ProfileSize: u32,
    pub bV5Reserved: u32,
}
pub const BITSPIXEL: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(12u32);
pub const BI_BITFIELDS: BI_COMPRESSION = BI_COMPRESSION(3u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BI_COMPRESSION(pub u32);
pub const BI_JPEG: BI_COMPRESSION = BI_COMPRESSION(4u32);
pub const BI_PNG: BI_COMPRESSION = BI_COMPRESSION(5u32);
pub const BI_RGB: BI_COMPRESSION = BI_COMPRESSION(0u32);
pub const BI_RLE4: BI_COMPRESSION = BI_COMPRESSION(2u32);
pub const BI_RLE8: BI_COMPRESSION = BI_COMPRESSION(1u32);
pub const BKMODE_LAST: u32 = 2u32;
pub const BLACKNESS: ROP_CODE = ROP_CODE(66u32);
pub const BLACKONWHITE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(1i32);
pub const BLACK_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(4i32);
pub const BLACK_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BLENDFUNCTION {
    pub BlendOp: u8,
    pub BlendFlags: u8,
    pub SourceConstantAlpha: u8,
    pub AlphaFormat: u8,
}
pub const BLTALIGNMENT: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(119u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BRUSH_STYLE(pub u32);
pub const BS_DIBPATTERN: BRUSH_STYLE = BRUSH_STYLE(5u32);
pub const BS_DIBPATTERN8X8: BRUSH_STYLE = BRUSH_STYLE(8u32);
pub const BS_DIBPATTERNPT: BRUSH_STYLE = BRUSH_STYLE(6u32);
pub const BS_HATCHED: BRUSH_STYLE = BRUSH_STYLE(2u32);
pub const BS_HOLLOW: BRUSH_STYLE = BRUSH_STYLE(1u32);
pub const BS_INDEXED: BRUSH_STYLE = BRUSH_STYLE(4u32);
pub const BS_MONOPATTERN: BRUSH_STYLE = BRUSH_STYLE(9u32);
pub const BS_NULL: BRUSH_STYLE = BRUSH_STYLE(1u32);
pub const BS_PATTERN: BRUSH_STYLE = BRUSH_STYLE(3u32);
pub const BS_PATTERN8X8: BRUSH_STYLE = BRUSH_STYLE(7u32);
pub const BS_SOLID: BRUSH_STYLE = BRUSH_STYLE(0u32);
pub const CAPTUREBLT: ROP_CODE = ROP_CODE(1073741824u32);
pub const CA_LOG_FILTER: u32 = 2u32;
pub const CA_NEGATIVE: u32 = 1u32;
pub const CBM_INIT: i32 = 4i32;
pub const CCHFORMNAME: u32 = 32u32;
pub const CC_CHORD: u32 = 4u32;
pub const CC_CIRCLES: u32 = 1u32;
pub const CC_ELLIPSES: u32 = 8u32;
pub const CC_INTERIORS: u32 = 128u32;
pub const CC_NONE: u32 = 0u32;
pub const CC_PIE: u32 = 2u32;
pub const CC_ROUNDRECT: u32 = 256u32;
pub const CC_STYLED: u32 = 32u32;
pub const CC_WIDE: u32 = 16u32;
pub const CC_WIDESTYLED: u32 = 64u32;
pub const CDS_DISABLE_UNSAFE_MODES: CDS_TYPE = CDS_TYPE(512u32);
pub const CDS_ENABLE_UNSAFE_MODES: CDS_TYPE = CDS_TYPE(256u32);
pub const CDS_FULLSCREEN: CDS_TYPE = CDS_TYPE(4u32);
pub const CDS_GLOBAL: CDS_TYPE = CDS_TYPE(8u32);
pub const CDS_NORESET: CDS_TYPE = CDS_TYPE(268435456u32);
pub const CDS_RESET: CDS_TYPE = CDS_TYPE(1073741824u32);
pub const CDS_RESET_EX: CDS_TYPE = CDS_TYPE(536870912u32);
pub const CDS_SET_PRIMARY: CDS_TYPE = CDS_TYPE(16u32);
pub const CDS_TEST: CDS_TYPE = CDS_TYPE(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDS_TYPE(pub u32);
impl CDS_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CDS_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CDS_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CDS_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CDS_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CDS_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CDS_UPDATEREGISTRY: CDS_TYPE = CDS_TYPE(1u32);
pub const CDS_VIDEOPARAMETERS: CDS_TYPE = CDS_TYPE(32u32);
pub type CFP_ALLOCPROC = Option<unsafe extern "system" fn(param0: usize) -> *mut core::ffi::c_void>;
pub type CFP_FREEPROC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
pub type CFP_REALLOCPROC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: usize) -> *mut core::ffi::c_void>;
pub const CHARSET_DEFAULT: u32 = 1u32;
pub const CHARSET_GLYPHIDX: u32 = 3u32;
pub const CHARSET_SYMBOL: EMBED_FONT_CHARSET = EMBED_FONT_CHARSET(2u32);
pub const CHARSET_UNICODE: EMBED_FONT_CHARSET = EMBED_FONT_CHARSET(1u32);
pub const CHECKJPEGFORMAT: u32 = 4119u32;
pub const CHECKPNGFORMAT: u32 = 4120u32;
pub const CHINESEBIG5_CHARSET: FONT_CHARSET = FONT_CHARSET(136u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CIEXYZ {
    pub ciexyzX: i32,
    pub ciexyzY: i32,
    pub ciexyzZ: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CIEXYZTRIPLE {
    pub ciexyzRed: CIEXYZ,
    pub ciexyzGreen: CIEXYZ,
    pub ciexyzBlue: CIEXYZ,
}
pub const CLEARTYPE_NATURAL_QUALITY: u32 = 6u32;
pub const CLEARTYPE_QUALITY: FONT_QUALITY = FONT_QUALITY(5u8);
pub const CLIPCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(36u32);
pub const CLIP_CHARACTER_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(1u8);
pub const CLIP_DEFAULT_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(0u8);
pub const CLIP_DFA_DISABLE: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(64u8);
pub const CLIP_DFA_OVERRIDE: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(64u8);
pub const CLIP_EMBEDDED: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(128u8);
pub const CLIP_LH_ANGLES: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(16u8);
pub const CLIP_MASK: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(15u8);
pub const CLIP_STROKE_PRECIS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(2u8);
pub const CLIP_TO_PATH: u32 = 4097u32;
pub const CLIP_TT_ALWAYS: FONT_CLIP_PRECISION = FONT_CLIP_PRECISION(32u8);
pub const CLOSECHANNEL: u32 = 4112u32;
pub const CLR_INVALID: u32 = 4294967295u32;
pub const CM_CMYK_COLOR: u32 = 4u32;
pub const CM_DEVICE_ICM: u32 = 1u32;
pub const CM_GAMMA_RAMP: u32 = 2u32;
pub const CM_IN_GAMUT: u32 = 0u32;
pub const CM_NONE: u32 = 0u32;
pub const CM_OUT_OF_GAMUT: u32 = 255u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORADJUSTMENT {
    pub caSize: u16,
    pub caFlags: u16,
    pub caIlluminantIndex: u16,
    pub caRedGamma: u16,
    pub caGreenGamma: u16,
    pub caBlueGamma: u16,
    pub caReferenceBlack: u16,
    pub caReferenceWhite: u16,
    pub caContrast: i16,
    pub caBrightness: i16,
    pub caColorfulness: i16,
    pub caRedGreenTint: i16,
}
pub const COLORMATCHTOTARGET_EMBEDED: u32 = 1u32;
pub const COLORMGMTCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(121u32);
pub const COLORONCOLOR: STRETCH_BLT_MODE = STRETCH_BLT_MODE(3i32);
pub const COLORRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(108u32);
pub const COLOR_3DDKSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(21i32);
pub const COLOR_3DFACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(15i32);
pub const COLOR_3DHIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20i32);
pub const COLOR_3DHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20i32);
pub const COLOR_3DLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(22i32);
pub const COLOR_3DSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(16i32);
pub const COLOR_ACTIVEBORDER: SYS_COLOR_INDEX = SYS_COLOR_INDEX(10i32);
pub const COLOR_ACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(2i32);
pub const COLOR_APPWORKSPACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(12i32);
pub const COLOR_BACKGROUND: SYS_COLOR_INDEX = SYS_COLOR_INDEX(1i32);
pub const COLOR_BTNFACE: SYS_COLOR_INDEX = SYS_COLOR_INDEX(15i32);
pub const COLOR_BTNHIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20i32);
pub const COLOR_BTNHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(20i32);
pub const COLOR_BTNSHADOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(16i32);
pub const COLOR_BTNTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(18i32);
pub const COLOR_CAPTIONTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(9i32);
pub const COLOR_DESKTOP: SYS_COLOR_INDEX = SYS_COLOR_INDEX(1i32);
pub const COLOR_GRADIENTACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(27i32);
pub const COLOR_GRADIENTINACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(28i32);
pub const COLOR_GRAYTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(17i32);
pub const COLOR_HIGHLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(13i32);
pub const COLOR_HIGHLIGHTTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(14i32);
pub const COLOR_HOTLIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(26i32);
pub const COLOR_INACTIVEBORDER: SYS_COLOR_INDEX = SYS_COLOR_INDEX(11i32);
pub const COLOR_INACTIVECAPTION: SYS_COLOR_INDEX = SYS_COLOR_INDEX(3i32);
pub const COLOR_INACTIVECAPTIONTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(19i32);
pub const COLOR_INFOBK: SYS_COLOR_INDEX = SYS_COLOR_INDEX(24i32);
pub const COLOR_INFOTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(23i32);
pub const COLOR_MENU: SYS_COLOR_INDEX = SYS_COLOR_INDEX(4i32);
pub const COLOR_MENUBAR: SYS_COLOR_INDEX = SYS_COLOR_INDEX(30i32);
pub const COLOR_MENUHILIGHT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(29i32);
pub const COLOR_MENUTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(7i32);
pub const COLOR_SCROLLBAR: SYS_COLOR_INDEX = SYS_COLOR_INDEX(0i32);
pub const COLOR_WINDOW: SYS_COLOR_INDEX = SYS_COLOR_INDEX(5i32);
pub const COLOR_WINDOWFRAME: SYS_COLOR_INDEX = SYS_COLOR_INDEX(6i32);
pub const COLOR_WINDOWTEXT: SYS_COLOR_INDEX = SYS_COLOR_INDEX(8i32);
pub const COMPLEXREGION: GDI_REGION_TYPE = GDI_REGION_TYPE(3i32);
pub const CP_NONE: u32 = 0u32;
pub const CP_RECTANGLE: u32 = 1u32;
pub const CP_REGION: u32 = 2u32;
pub const CREATECOLORSPACE_EMBEDED: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_FONT_PACKAGE_SUBSET_ENCODING(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_FONT_PACKAGE_SUBSET_PLATFORM(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_POLYGON_RGN_MODE(pub i32);
pub const CURVECAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(28u32);
pub const DCBA_FACEDOWNCENTER: u32 = 257u32;
pub const DCBA_FACEDOWNLEFT: u32 = 258u32;
pub const DCBA_FACEDOWNNONE: u32 = 256u32;
pub const DCBA_FACEDOWNRIGHT: u32 = 259u32;
pub const DCBA_FACEUPCENTER: u32 = 1u32;
pub const DCBA_FACEUPLEFT: u32 = 2u32;
pub const DCBA_FACEUPNONE: u32 = 0u32;
pub const DCBA_FACEUPRIGHT: u32 = 3u32;
pub const DCB_ACCUMULATE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(2u32);
pub const DCB_DISABLE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(8u32);
pub const DCB_ENABLE: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(4u32);
pub const DCB_RESET: SET_BOUNDS_RECT_FLAGS = SET_BOUNDS_RECT_FLAGS(1u32);
pub const DCTT_BITMAP: i32 = 1i32;
pub const DCTT_DOWNLOAD: i32 = 2i32;
pub const DCTT_DOWNLOAD_OUTLINE: i32 = 8i32;
pub const DCTT_SUBDEV: i32 = 4i32;
pub const DCX_CACHE: GET_DCX_FLAGS = GET_DCX_FLAGS(2u32);
pub const DCX_CLIPCHILDREN: GET_DCX_FLAGS = GET_DCX_FLAGS(8u32);
pub const DCX_CLIPSIBLINGS: GET_DCX_FLAGS = GET_DCX_FLAGS(16u32);
pub const DCX_EXCLUDERGN: GET_DCX_FLAGS = GET_DCX_FLAGS(64u32);
pub const DCX_INTERSECTRGN: GET_DCX_FLAGS = GET_DCX_FLAGS(128u32);
pub const DCX_INTERSECTUPDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(512u32);
pub const DCX_LOCKWINDOWUPDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(1024u32);
pub const DCX_NORESETATTRS: GET_DCX_FLAGS = GET_DCX_FLAGS(4u32);
pub const DCX_PARENTCLIP: GET_DCX_FLAGS = GET_DCX_FLAGS(32u32);
pub const DCX_VALIDATE: GET_DCX_FLAGS = GET_DCX_FLAGS(2097152u32);
pub const DCX_WINDOW: GET_DCX_FLAGS = GET_DCX_FLAGS(1u32);
pub const DC_ACTIVE: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(1u32);
pub const DC_BINADJUST: u32 = 19u32;
pub const DC_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(18i32);
pub const DC_BUTTONS: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(4096u32);
pub const DC_DATATYPE_PRODUCED: u32 = 21u32;
pub const DC_EMF_COMPLIANT: u32 = 20u32;
pub const DC_GRADIENT: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(32u32);
pub const DC_ICON: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(4u32);
pub const DC_INBUTTON: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(16u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DC_LAYOUT(pub u32);
impl DC_LAYOUT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DC_LAYOUT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DC_LAYOUT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DC_LAYOUT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DC_LAYOUT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DC_LAYOUT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DC_MANUFACTURER: u32 = 23u32;
pub const DC_MODEL: u32 = 24u32;
pub const DC_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(19i32);
pub const DC_SMALLCAP: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(2u32);
pub const DC_TEXT: DRAW_CAPTION_FLAGS = DRAW_CAPTION_FLAGS(8u32);
pub const DEFAULT_CHARSET: FONT_CHARSET = FONT_CHARSET(1u8);
pub const DEFAULT_GUI_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(17i32);
pub const DEFAULT_PALETTE: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(15i32);
pub const DEFAULT_PITCH: FONT_PITCH = FONT_PITCH(0u8);
pub const DEFAULT_QUALITY: FONT_QUALITY = FONT_QUALITY(0u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DESIGNVECTOR {
    pub dvReserved: u32,
    pub dvNumAxes: u32,
    pub dvValues: [i32; 16],
}
impl Default for DESIGNVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DESKTOPHORZRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(118u32);
pub const DESKTOPVERTRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(117u32);
pub const DEVICEDATA: u32 = 19u32;
pub const DEVICE_DEFAULT_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(14i32);
pub const DEVICE_FONTTYPE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVMODEA {
    pub dmDeviceName: [u8; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: DEVMODE_FIELD_FLAGS,
    pub Anonymous1: DEVMODEA_0,
    pub dmColor: DEVMODE_COLOR,
    pub dmDuplex: DEVMODE_DUPLEX,
    pub dmYResolution: i16,
    pub dmTTOption: DEVMODE_TRUETYPE_OPTION,
    pub dmCollate: DEVMODE_COLLATE,
    pub dmFormName: [u8; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEA_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
impl Default for DEVMODEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVMODEA_0 {
    pub Anonymous1: DEVMODEA_0_0,
    pub Anonymous2: DEVMODEA_0_1,
}
impl Default for DEVMODEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVMODEA_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVMODEA_0_1 {
    pub dmPosition: super::super::Foundation::POINTL,
    pub dmDisplayOrientation: DEVMODE_DISPLAY_ORIENTATION,
    pub dmDisplayFixedOutput: DEVMODE_DISPLAY_FIXED_OUTPUT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVMODEA_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
impl Default for DEVMODEA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVMODEW {
    pub dmDeviceName: [u16; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: DEVMODE_FIELD_FLAGS,
    pub Anonymous1: DEVMODEW_0,
    pub dmColor: DEVMODE_COLOR,
    pub dmDuplex: DEVMODE_DUPLEX,
    pub dmYResolution: i16,
    pub dmTTOption: DEVMODE_TRUETYPE_OPTION,
    pub dmCollate: DEVMODE_COLLATE,
    pub dmFormName: [u16; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEW_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
impl Default for DEVMODEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVMODEW_0 {
    pub Anonymous1: DEVMODEW_0_0,
    pub Anonymous2: DEVMODEW_0_1,
}
impl Default for DEVMODEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVMODEW_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVMODEW_0_1 {
    pub dmPosition: super::super::Foundation::POINTL,
    pub dmDisplayOrientation: DEVMODE_DISPLAY_ORIENTATION,
    pub dmDisplayFixedOutput: DEVMODE_DISPLAY_FIXED_OUTPUT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVMODEW_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
impl Default for DEVMODEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVMODE_COLLATE(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVMODE_COLOR(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVMODE_DISPLAY_FIXED_OUTPUT(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVMODE_DISPLAY_ORIENTATION(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVMODE_DUPLEX(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVMODE_FIELD_FLAGS(pub u32);
impl DEVMODE_FIELD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DEVMODE_FIELD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DEVMODE_FIELD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVMODE_TRUETYPE_OPTION(pub i16);
pub const DFCS_ADJUSTRECT: DFCS_STATE = DFCS_STATE(8192u32);
pub const DFCS_BUTTON3STATE: DFCS_STATE = DFCS_STATE(8u32);
pub const DFCS_BUTTONCHECK: DFCS_STATE = DFCS_STATE(0u32);
pub const DFCS_BUTTONPUSH: DFCS_STATE = DFCS_STATE(16u32);
pub const DFCS_BUTTONRADIO: DFCS_STATE = DFCS_STATE(4u32);
pub const DFCS_BUTTONRADIOIMAGE: DFCS_STATE = DFCS_STATE(1u32);
pub const DFCS_BUTTONRADIOMASK: DFCS_STATE = DFCS_STATE(2u32);
pub const DFCS_CAPTIONCLOSE: DFCS_STATE = DFCS_STATE(0u32);
pub const DFCS_CAPTIONHELP: DFCS_STATE = DFCS_STATE(4u32);
pub const DFCS_CAPTIONMAX: DFCS_STATE = DFCS_STATE(2u32);
pub const DFCS_CAPTIONMIN: DFCS_STATE = DFCS_STATE(1u32);
pub const DFCS_CAPTIONRESTORE: DFCS_STATE = DFCS_STATE(3u32);
pub const DFCS_CHECKED: DFCS_STATE = DFCS_STATE(1024u32);
pub const DFCS_FLAT: DFCS_STATE = DFCS_STATE(16384u32);
pub const DFCS_HOT: DFCS_STATE = DFCS_STATE(4096u32);
pub const DFCS_INACTIVE: DFCS_STATE = DFCS_STATE(256u32);
pub const DFCS_MENUARROW: DFCS_STATE = DFCS_STATE(0u32);
pub const DFCS_MENUARROWRIGHT: DFCS_STATE = DFCS_STATE(4u32);
pub const DFCS_MENUBULLET: DFCS_STATE = DFCS_STATE(2u32);
pub const DFCS_MENUCHECK: DFCS_STATE = DFCS_STATE(1u32);
pub const DFCS_MONO: DFCS_STATE = DFCS_STATE(32768u32);
pub const DFCS_PUSHED: DFCS_STATE = DFCS_STATE(512u32);
pub const DFCS_SCROLLCOMBOBOX: DFCS_STATE = DFCS_STATE(5u32);
pub const DFCS_SCROLLDOWN: DFCS_STATE = DFCS_STATE(1u32);
pub const DFCS_SCROLLLEFT: DFCS_STATE = DFCS_STATE(2u32);
pub const DFCS_SCROLLRIGHT: DFCS_STATE = DFCS_STATE(3u32);
pub const DFCS_SCROLLSIZEGRIP: DFCS_STATE = DFCS_STATE(8u32);
pub const DFCS_SCROLLSIZEGRIPRIGHT: DFCS_STATE = DFCS_STATE(16u32);
pub const DFCS_SCROLLUP: DFCS_STATE = DFCS_STATE(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DFCS_STATE(pub u32);
impl DFCS_STATE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DFCS_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DFCS_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DFCS_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DFCS_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DFCS_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DFCS_TRANSPARENT: DFCS_STATE = DFCS_STATE(2048u32);
pub const DFC_BUTTON: DFC_TYPE = DFC_TYPE(4u32);
pub const DFC_CAPTION: DFC_TYPE = DFC_TYPE(1u32);
pub const DFC_MENU: DFC_TYPE = DFC_TYPE(2u32);
pub const DFC_POPUPMENU: DFC_TYPE = DFC_TYPE(5u32);
pub const DFC_SCROLL: DFC_TYPE = DFC_TYPE(3u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DFC_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIBSECTION {
    pub dsBm: BITMAP,
    pub dsBmih: BITMAPINFOHEADER,
    pub dsBitfields: [u32; 3],
    pub dshSection: super::super::Foundation::HANDLE,
    pub dsOffset: u32,
}
impl Default for DIBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIB_PAL_COLORS: DIB_USAGE = DIB_USAGE(1u32);
pub const DIB_RGB_COLORS: DIB_USAGE = DIB_USAGE(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIB_USAGE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_COLOR_ENCODING(pub i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_INTENSITY: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(4i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_RGB: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(0i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR420: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(3i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR422: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(2i32);
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR444: DISPLAYCONFIG_COLOR_ENCODING = DISPLAYCONFIG_COLOR_ENCODING(1i32);
pub const DISPLAYCONFIG_MAXPATH: u32 = 1024u32;
pub const DISPLAYCONFIG_PATH_ACTIVE: u32 = 1u32;
pub const DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: u32 = 4294967295u32;
pub const DISPLAYCONFIG_PATH_PREFERRED_UNSCALED: u32 = 4u32;
pub const DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE: u32 = 8u32;
pub const DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_VALID_FLAGS: u32 = 29u32;
pub const DISPLAYCONFIG_SOURCE_IN_USE: u32 = 1u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: u32 = 4u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: u32 = 8u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: u32 = 16u32;
pub const DISPLAYCONFIG_TARGET_FORCIBLE: u32 = 2u32;
pub const DISPLAYCONFIG_TARGET_IN_USE: u32 = 1u32;
pub const DISPLAYCONFIG_TARGET_IS_HMD: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPLAY_DEVICEA {
    pub cb: u32,
    pub DeviceName: [i8; 32],
    pub DeviceString: [i8; 128],
    pub StateFlags: DISPLAY_DEVICE_STATE_FLAGS,
    pub DeviceID: [i8; 128],
    pub DeviceKey: [i8; 128],
}
impl Default for DISPLAY_DEVICEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPLAY_DEVICEW {
    pub cb: u32,
    pub DeviceName: [u16; 32],
    pub DeviceString: [u16; 128],
    pub StateFlags: DISPLAY_DEVICE_STATE_FLAGS,
    pub DeviceID: [u16; 128],
    pub DeviceKey: [u16; 128],
}
impl Default for DISPLAY_DEVICEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAY_DEVICE_ACC_DRIVER: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(64u32);
pub const DISPLAY_DEVICE_ACTIVE: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(1u32);
pub const DISPLAY_DEVICE_ATTACHED: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(2u32);
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(1u32);
pub const DISPLAY_DEVICE_DISCONNECT: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(33554432u32);
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(8u32);
pub const DISPLAY_DEVICE_MODESPRUNED: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(134217728u32);
pub const DISPLAY_DEVICE_MULTI_DRIVER: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(2u32);
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(4u32);
pub const DISPLAY_DEVICE_RDPUDD: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(16777216u32);
pub const DISPLAY_DEVICE_REMOTE: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(67108864u32);
pub const DISPLAY_DEVICE_REMOVABLE: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(32u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAY_DEVICE_STATE_FLAGS(pub u32);
impl DISPLAY_DEVICE_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DISPLAY_DEVICE_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DISPLAY_DEVICE_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DISPLAY_DEVICE_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DISPLAY_DEVICE_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DISPLAY_DEVICE_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DISPLAY_DEVICE_TS_COMPATIBLE: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(2097152u32);
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(524288u32);
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: DISPLAY_DEVICE_STATE_FLAGS = DISPLAY_DEVICE_STATE_FLAGS(16u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISP_CHANGE(pub i32);
pub const DISP_CHANGE_BADDUALVIEW: DISP_CHANGE = DISP_CHANGE(-6i32);
pub const DISP_CHANGE_BADFLAGS: DISP_CHANGE = DISP_CHANGE(-4i32);
pub const DISP_CHANGE_BADMODE: DISP_CHANGE = DISP_CHANGE(-2i32);
pub const DISP_CHANGE_BADPARAM: DISP_CHANGE = DISP_CHANGE(-5i32);
pub const DISP_CHANGE_FAILED: DISP_CHANGE = DISP_CHANGE(-1i32);
pub const DISP_CHANGE_NOTUPDATED: DISP_CHANGE = DISP_CHANGE(-3i32);
pub const DISP_CHANGE_RESTART: DISP_CHANGE = DISP_CHANGE(1i32);
pub const DISP_CHANGE_SUCCESSFUL: DISP_CHANGE = DISP_CHANGE(0i32);
pub const DI_APPBANDING: u32 = 1u32;
pub const DI_ROPS_READ_DESTINATION: u32 = 2u32;
pub const DKGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(3i32);
pub const DMBIN_AUTO: u32 = 7u32;
pub const DMBIN_CASSETTE: u32 = 14u32;
pub const DMBIN_ENVELOPE: u32 = 5u32;
pub const DMBIN_ENVMANUAL: u32 = 6u32;
pub const DMBIN_FORMSOURCE: u32 = 15u32;
pub const DMBIN_LARGECAPACITY: u32 = 11u32;
pub const DMBIN_LARGEFMT: u32 = 10u32;
pub const DMBIN_LAST: u32 = 15u32;
pub const DMBIN_LOWER: u32 = 2u32;
pub const DMBIN_MANUAL: u32 = 4u32;
pub const DMBIN_MIDDLE: u32 = 3u32;
pub const DMBIN_ONLYONE: u32 = 1u32;
pub const DMBIN_SMALLFMT: u32 = 9u32;
pub const DMBIN_TRACTOR: u32 = 8u32;
pub const DMBIN_UPPER: u32 = 1u32;
pub const DMBIN_USER: u32 = 256u32;
pub const DMCOLLATE_FALSE: DEVMODE_COLLATE = DEVMODE_COLLATE(0i16);
pub const DMCOLLATE_TRUE: DEVMODE_COLLATE = DEVMODE_COLLATE(1i16);
pub const DMCOLOR_COLOR: DEVMODE_COLOR = DEVMODE_COLOR(2i16);
pub const DMCOLOR_MONOCHROME: DEVMODE_COLOR = DEVMODE_COLOR(1i16);
pub const DMDFO_CENTER: DEVMODE_DISPLAY_FIXED_OUTPUT = DEVMODE_DISPLAY_FIXED_OUTPUT(2u32);
pub const DMDFO_DEFAULT: DEVMODE_DISPLAY_FIXED_OUTPUT = DEVMODE_DISPLAY_FIXED_OUTPUT(0u32);
pub const DMDFO_STRETCH: DEVMODE_DISPLAY_FIXED_OUTPUT = DEVMODE_DISPLAY_FIXED_OUTPUT(1u32);
pub const DMDISPLAYFLAGS_TEXTMODE: u32 = 4u32;
pub const DMDITHER_COARSE: u32 = 2u32;
pub const DMDITHER_ERRORDIFFUSION: u32 = 5u32;
pub const DMDITHER_FINE: u32 = 3u32;
pub const DMDITHER_GRAYSCALE: u32 = 10u32;
pub const DMDITHER_LINEART: u32 = 4u32;
pub const DMDITHER_NONE: u32 = 1u32;
pub const DMDITHER_RESERVED6: u32 = 6u32;
pub const DMDITHER_RESERVED7: u32 = 7u32;
pub const DMDITHER_RESERVED8: u32 = 8u32;
pub const DMDITHER_RESERVED9: u32 = 9u32;
pub const DMDITHER_USER: u32 = 256u32;
pub const DMDO_180: DEVMODE_DISPLAY_ORIENTATION = DEVMODE_DISPLAY_ORIENTATION(2u32);
pub const DMDO_270: DEVMODE_DISPLAY_ORIENTATION = DEVMODE_DISPLAY_ORIENTATION(3u32);
pub const DMDO_90: DEVMODE_DISPLAY_ORIENTATION = DEVMODE_DISPLAY_ORIENTATION(1u32);
pub const DMDO_DEFAULT: DEVMODE_DISPLAY_ORIENTATION = DEVMODE_DISPLAY_ORIENTATION(0u32);
pub const DMDUP_HORIZONTAL: DEVMODE_DUPLEX = DEVMODE_DUPLEX(3i16);
pub const DMDUP_SIMPLEX: DEVMODE_DUPLEX = DEVMODE_DUPLEX(1i16);
pub const DMDUP_VERTICAL: DEVMODE_DUPLEX = DEVMODE_DUPLEX(2i16);
pub const DMICMMETHOD_DEVICE: u32 = 4u32;
pub const DMICMMETHOD_DRIVER: u32 = 3u32;
pub const DMICMMETHOD_NONE: u32 = 1u32;
pub const DMICMMETHOD_SYSTEM: u32 = 2u32;
pub const DMICMMETHOD_USER: u32 = 256u32;
pub const DMICM_ABS_COLORIMETRIC: u32 = 4u32;
pub const DMICM_COLORIMETRIC: u32 = 3u32;
pub const DMICM_CONTRAST: u32 = 2u32;
pub const DMICM_SATURATE: u32 = 1u32;
pub const DMICM_USER: u32 = 256u32;
pub const DMMEDIA_GLOSSY: u32 = 3u32;
pub const DMMEDIA_STANDARD: u32 = 1u32;
pub const DMMEDIA_TRANSPARENCY: u32 = 2u32;
pub const DMMEDIA_USER: u32 = 256u32;
pub const DMNUP_ONEUP: u32 = 2u32;
pub const DMNUP_SYSTEM: u32 = 1u32;
pub const DMORIENT_LANDSCAPE: u32 = 2u32;
pub const DMORIENT_PORTRAIT: u32 = 1u32;
pub const DMPAPER_10X11: u32 = 45u32;
pub const DMPAPER_10X14: u32 = 16u32;
pub const DMPAPER_11X17: u32 = 17u32;
pub const DMPAPER_12X11: u32 = 90u32;
pub const DMPAPER_15X11: u32 = 46u32;
pub const DMPAPER_9X11: u32 = 44u32;
pub const DMPAPER_A2: u32 = 66u32;
pub const DMPAPER_A3: u32 = 8u32;
pub const DMPAPER_A3_EXTRA: u32 = 63u32;
pub const DMPAPER_A3_EXTRA_TRANSVERSE: u32 = 68u32;
pub const DMPAPER_A3_ROTATED: u32 = 76u32;
pub const DMPAPER_A3_TRANSVERSE: u32 = 67u32;
pub const DMPAPER_A4: u32 = 9u32;
pub const DMPAPER_A4SMALL: u32 = 10u32;
pub const DMPAPER_A4_EXTRA: u32 = 53u32;
pub const DMPAPER_A4_PLUS: u32 = 60u32;
pub const DMPAPER_A4_ROTATED: u32 = 77u32;
pub const DMPAPER_A4_TRANSVERSE: u32 = 55u32;
pub const DMPAPER_A5: u32 = 11u32;
pub const DMPAPER_A5_EXTRA: u32 = 64u32;
pub const DMPAPER_A5_ROTATED: u32 = 78u32;
pub const DMPAPER_A5_TRANSVERSE: u32 = 61u32;
pub const DMPAPER_A6: u32 = 70u32;
pub const DMPAPER_A6_ROTATED: u32 = 83u32;
pub const DMPAPER_A_PLUS: u32 = 57u32;
pub const DMPAPER_B4: u32 = 12u32;
pub const DMPAPER_B4_JIS_ROTATED: u32 = 79u32;
pub const DMPAPER_B5: u32 = 13u32;
pub const DMPAPER_B5_EXTRA: u32 = 65u32;
pub const DMPAPER_B5_JIS_ROTATED: u32 = 80u32;
pub const DMPAPER_B5_TRANSVERSE: u32 = 62u32;
pub const DMPAPER_B6_JIS: u32 = 88u32;
pub const DMPAPER_B6_JIS_ROTATED: u32 = 89u32;
pub const DMPAPER_B_PLUS: u32 = 58u32;
pub const DMPAPER_CSHEET: u32 = 24u32;
pub const DMPAPER_DBL_JAPANESE_POSTCARD: u32 = 69u32;
pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: u32 = 82u32;
pub const DMPAPER_DSHEET: u32 = 25u32;
pub const DMPAPER_ENV_10: u32 = 20u32;
pub const DMPAPER_ENV_11: u32 = 21u32;
pub const DMPAPER_ENV_12: u32 = 22u32;
pub const DMPAPER_ENV_14: u32 = 23u32;
pub const DMPAPER_ENV_9: u32 = 19u32;
pub const DMPAPER_ENV_B4: u32 = 33u32;
pub const DMPAPER_ENV_B5: u32 = 34u32;
pub const DMPAPER_ENV_B6: u32 = 35u32;
pub const DMPAPER_ENV_C3: u32 = 29u32;
pub const DMPAPER_ENV_C4: u32 = 30u32;
pub const DMPAPER_ENV_C5: u32 = 28u32;
pub const DMPAPER_ENV_C6: u32 = 31u32;
pub const DMPAPER_ENV_C65: u32 = 32u32;
pub const DMPAPER_ENV_DL: u32 = 27u32;
pub const DMPAPER_ENV_INVITE: u32 = 47u32;
pub const DMPAPER_ENV_ITALY: u32 = 36u32;
pub const DMPAPER_ENV_MONARCH: u32 = 37u32;
pub const DMPAPER_ENV_PERSONAL: u32 = 38u32;
pub const DMPAPER_ESHEET: u32 = 26u32;
pub const DMPAPER_EXECUTIVE: u32 = 7u32;
pub const DMPAPER_FANFOLD_LGL_GERMAN: u32 = 41u32;
pub const DMPAPER_FANFOLD_STD_GERMAN: u32 = 40u32;
pub const DMPAPER_FANFOLD_US: u32 = 39u32;
pub const DMPAPER_FOLIO: u32 = 14u32;
pub const DMPAPER_ISO_B4: u32 = 42u32;
pub const DMPAPER_JAPANESE_POSTCARD: u32 = 43u32;
pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: u32 = 81u32;
pub const DMPAPER_JENV_CHOU3: u32 = 73u32;
pub const DMPAPER_JENV_CHOU3_ROTATED: u32 = 86u32;
pub const DMPAPER_JENV_CHOU4: u32 = 74u32;
pub const DMPAPER_JENV_CHOU4_ROTATED: u32 = 87u32;
pub const DMPAPER_JENV_KAKU2: u32 = 71u32;
pub const DMPAPER_JENV_KAKU2_ROTATED: u32 = 84u32;
pub const DMPAPER_JENV_KAKU3: u32 = 72u32;
pub const DMPAPER_JENV_KAKU3_ROTATED: u32 = 85u32;
pub const DMPAPER_JENV_YOU4: u32 = 91u32;
pub const DMPAPER_JENV_YOU4_ROTATED: u32 = 92u32;
pub const DMPAPER_LAST: u32 = 118u32;
pub const DMPAPER_LEDGER: u32 = 4u32;
pub const DMPAPER_LEGAL: u32 = 5u32;
pub const DMPAPER_LEGAL_EXTRA: u32 = 51u32;
pub const DMPAPER_LETTER: u32 = 1u32;
pub const DMPAPER_LETTERSMALL: u32 = 2u32;
pub const DMPAPER_LETTER_EXTRA: u32 = 50u32;
pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: u32 = 56u32;
pub const DMPAPER_LETTER_PLUS: u32 = 59u32;
pub const DMPAPER_LETTER_ROTATED: u32 = 75u32;
pub const DMPAPER_LETTER_TRANSVERSE: u32 = 54u32;
pub const DMPAPER_NOTE: u32 = 18u32;
pub const DMPAPER_P16K: u32 = 93u32;
pub const DMPAPER_P16K_ROTATED: u32 = 106u32;
pub const DMPAPER_P32K: u32 = 94u32;
pub const DMPAPER_P32KBIG: u32 = 95u32;
pub const DMPAPER_P32KBIG_ROTATED: u32 = 108u32;
pub const DMPAPER_P32K_ROTATED: u32 = 107u32;
pub const DMPAPER_PENV_1: u32 = 96u32;
pub const DMPAPER_PENV_10: u32 = 105u32;
pub const DMPAPER_PENV_10_ROTATED: u32 = 118u32;
pub const DMPAPER_PENV_1_ROTATED: u32 = 109u32;
pub const DMPAPER_PENV_2: u32 = 97u32;
pub const DMPAPER_PENV_2_ROTATED: u32 = 110u32;
pub const DMPAPER_PENV_3: u32 = 98u32;
pub const DMPAPER_PENV_3_ROTATED: u32 = 111u32;
pub const DMPAPER_PENV_4: u32 = 99u32;
pub const DMPAPER_PENV_4_ROTATED: u32 = 112u32;
pub const DMPAPER_PENV_5: u32 = 100u32;
pub const DMPAPER_PENV_5_ROTATED: u32 = 113u32;
pub const DMPAPER_PENV_6: u32 = 101u32;
pub const DMPAPER_PENV_6_ROTATED: u32 = 114u32;
pub const DMPAPER_PENV_7: u32 = 102u32;
pub const DMPAPER_PENV_7_ROTATED: u32 = 115u32;
pub const DMPAPER_PENV_8: u32 = 103u32;
pub const DMPAPER_PENV_8_ROTATED: u32 = 116u32;
pub const DMPAPER_PENV_9: u32 = 104u32;
pub const DMPAPER_PENV_9_ROTATED: u32 = 117u32;
pub const DMPAPER_QUARTO: u32 = 15u32;
pub const DMPAPER_RESERVED_48: u32 = 48u32;
pub const DMPAPER_RESERVED_49: u32 = 49u32;
pub const DMPAPER_STATEMENT: u32 = 6u32;
pub const DMPAPER_TABLOID: u32 = 3u32;
pub const DMPAPER_TABLOID_EXTRA: u32 = 52u32;
pub const DMPAPER_USER: u32 = 256u32;
pub const DMRES_DRAFT: i32 = -1i32;
pub const DMRES_HIGH: i32 = -4i32;
pub const DMRES_LOW: i32 = -2i32;
pub const DMRES_MEDIUM: i32 = -3i32;
pub const DMTT_BITMAP: DEVMODE_TRUETYPE_OPTION = DEVMODE_TRUETYPE_OPTION(1i16);
pub const DMTT_DOWNLOAD: DEVMODE_TRUETYPE_OPTION = DEVMODE_TRUETYPE_OPTION(2i16);
pub const DMTT_DOWNLOAD_OUTLINE: DEVMODE_TRUETYPE_OPTION = DEVMODE_TRUETYPE_OPTION(4i16);
pub const DMTT_SUBDEV: DEVMODE_TRUETYPE_OPTION = DEVMODE_TRUETYPE_OPTION(3i16);
pub const DM_BITSPERPEL: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(262144u32);
pub const DM_COLLATE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(32768u32);
pub const DM_COLOR: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2048u32);
pub const DM_COPIES: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(256u32);
pub const DM_COPY: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2u32);
pub const DM_DEFAULTSOURCE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(512u32);
pub const DM_DISPLAYFIXEDOUTPUT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(536870912u32);
pub const DM_DISPLAYFLAGS: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2097152u32);
pub const DM_DISPLAYFREQUENCY: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4194304u32);
pub const DM_DISPLAYORIENTATION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(128u32);
pub const DM_DITHERTYPE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(67108864u32);
pub const DM_DUPLEX: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4096u32);
pub const DM_FORMNAME: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(65536u32);
pub const DM_ICMINTENT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(16777216u32);
pub const DM_ICMMETHOD: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8388608u32);
pub const DM_INTERLACED: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2u32);
pub const DM_IN_BUFFER: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8u32);
pub const DM_IN_PROMPT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4u32);
pub const DM_LOGPIXELS: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(131072u32);
pub const DM_MEDIATYPE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(33554432u32);
pub const DM_MODIFY: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8u32);
pub const DM_NUP: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(64u32);
pub const DM_ORIENTATION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1u32);
pub const DM_OUT_BUFFER: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2u32);
pub const DM_OUT_DEFAULT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1u32);
pub const DM_PANNINGHEIGHT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(268435456u32);
pub const DM_PANNINGWIDTH: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(134217728u32);
pub const DM_PAPERLENGTH: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4u32);
pub const DM_PAPERSIZE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(2u32);
pub const DM_PAPERWIDTH: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8u32);
pub const DM_PELSHEIGHT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1048576u32);
pub const DM_PELSWIDTH: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(524288u32);
pub const DM_POSITION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(32u32);
pub const DM_PRINTQUALITY: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1024u32);
pub const DM_PROMPT: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(4u32);
pub const DM_SCALE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(16u32);
pub const DM_SPECVERSION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1025u32);
pub const DM_TTOPTION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(16384u32);
pub const DM_UPDATE: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(1u32);
pub const DM_YRESOLUTION: DEVMODE_FIELD_FLAGS = DEVMODE_FIELD_FLAGS(8192u32);
pub const DOWNLOADFACE: u32 = 514u32;
pub const DOWNLOADHEADER: u32 = 4111u32;
pub const DRAFTMODE: u32 = 7u32;
pub const DRAFT_QUALITY: FONT_QUALITY = FONT_QUALITY(1u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAWEDGE_FLAGS(pub u32);
impl DRAWEDGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DRAWEDGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DRAWEDGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DRAWEDGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DRAWEDGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DRAWEDGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DRAWPATTERNRECT: u32 = 25u32;
pub type DRAWSTATEPROC = Option<unsafe extern "system" fn(hdc: HDC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, cx: i32, cy: i32) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAWSTATE_FLAGS(pub u32);
impl DRAWSTATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DRAWSTATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DRAWSTATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DRAWSTATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DRAWSTATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DRAWSTATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRAWTEXTPARAMS {
    pub cbSize: u32,
    pub iTabLength: i32,
    pub iLeftMargin: i32,
    pub iRightMargin: i32,
    pub uiLengthDrawn: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAW_CAPTION_FLAGS(pub u32);
impl DRAW_CAPTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DRAW_CAPTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DRAW_CAPTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAW_EDGE_FLAGS(pub u32);
impl DRAW_EDGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DRAW_EDGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DRAW_EDGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAW_TEXT_FORMAT(pub u32);
impl DRAW_TEXT_FORMAT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DRAW_TEXT_FORMAT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DRAW_TEXT_FORMAT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DRIVERVERSION: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(0u32);
pub const DSS_DISABLED: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(32u32);
pub const DSS_HIDEPREFIX: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(512u32);
pub const DSS_MONO: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(128u32);
pub const DSS_NORMAL: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(0u32);
pub const DSS_PREFIXONLY: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(1024u32);
pub const DSS_RIGHT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(32768u32);
pub const DSS_UNION: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(16u32);
pub const DSTINVERT: ROP_CODE = ROP_CODE(5570569u32);
pub const DST_BITMAP: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(4u32);
pub const DST_COMPLEX: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(0u32);
pub const DST_ICON: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(3u32);
pub const DST_PREFIXTEXT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(2u32);
pub const DST_TEXT: DRAWSTATE_FLAGS = DRAWSTATE_FLAGS(1u32);
pub const DT_BOTTOM: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(8u32);
pub const DT_CALCRECT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1024u32);
pub const DT_CENTER: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1u32);
pub const DT_CHARSTREAM: u32 = 4u32;
pub const DT_DISPFILE: u32 = 6u32;
pub const DT_EDITCONTROL: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(8192u32);
pub const DT_END_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(32768u32);
pub const DT_EXPANDTABS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(64u32);
pub const DT_EXTERNALLEADING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(512u32);
pub const DT_HIDEPREFIX: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(1048576u32);
pub const DT_INTERNAL: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(4096u32);
pub const DT_LEFT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0u32);
pub const DT_METAFILE: u32 = 5u32;
pub const DT_MODIFYSTRING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(65536u32);
pub const DT_NOCLIP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(256u32);
pub const DT_NOFULLWIDTHCHARBREAK: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(524288u32);
pub const DT_NOPREFIX: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2048u32);
pub const DT_PATH_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(16384u32);
pub const DT_PLOTTER: u32 = 0u32;
pub const DT_PREFIXONLY: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2097152u32);
pub const DT_RASCAMERA: u32 = 3u32;
pub const DT_RASDISPLAY: u32 = 1u32;
pub const DT_RASPRINTER: u32 = 2u32;
pub const DT_RIGHT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(2u32);
pub const DT_RTLREADING: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(131072u32);
pub const DT_SINGLELINE: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(32u32);
pub const DT_TABSTOP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(128u32);
pub const DT_TOP: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0u32);
pub const DT_VCENTER: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(4u32);
pub const DT_WORDBREAK: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(16u32);
pub const DT_WORD_ELLIPSIS: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(262144u32);
pub const EASTEUROPE_CHARSET: FONT_CHARSET = FONT_CHARSET(238u8);
pub const EDGE_BUMP: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(9u32);
pub const EDGE_ETCHED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(6u32);
pub const EDGE_RAISED: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(5u32);
pub const EDGE_SUNKEN: DRAWEDGE_FLAGS = DRAWEDGE_FLAGS(10u32);
pub const EDS_RAWMODE: ENUM_DISPLAY_SETTINGS_FLAGS = ENUM_DISPLAY_SETTINGS_FLAGS(2u32);
pub const EDS_ROTATEDMODE: ENUM_DISPLAY_SETTINGS_FLAGS = ENUM_DISPLAY_SETTINGS_FLAGS(4u32);
pub const ELF_CULTURE_LATIN: u32 = 0u32;
pub const ELF_VENDOR_SIZE: u32 = 4u32;
pub const ELF_VERSION: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EMBEDDED_FONT_PRIV_STATUS(pub u32);
pub const EMBED_EDITABLE: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EMBED_FONT_CHARSET(pub u32);
pub const EMBED_INSTALLABLE: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(3u32);
pub const EMBED_NOEMBEDDING: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(4u32);
pub const EMBED_PREVIEWPRINT: EMBEDDED_FONT_PRIV_STATUS = EMBEDDED_FONT_PRIV_STATUS(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMR {
    pub iType: ENHANCED_METAFILE_RECORD_TYPE,
    pub nSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRALPHABLEND {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRANGLEARC {
    pub emr: EMR,
    pub ptlCenter: super::super::Foundation::POINTL,
    pub nRadius: u32,
    pub eStartAngle: f32,
    pub eSweepAngle: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRARC {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
    pub ptlStart: super::super::Foundation::POINTL,
    pub ptlEnd: super::super::Foundation::POINTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRBITBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRCOLORCORRECTPALETTE {
    pub emr: EMR,
    pub ihPalette: u32,
    pub nFirstEntry: u32,
    pub nPalEntries: u32,
    pub nReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRCOLORMATCHTOTARGET {
    pub emr: EMR,
    pub dwAction: u32,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRCOLORMATCHTOTARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRCREATEBRUSHINDIRECT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub lb: LOGBRUSH32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRCREATEDIBPATTERNBRUSHPT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRCREATEMONOBRUSH {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRCREATEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub lgpl: LOGPALETTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub lopn: LOGPEN,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRELLIPSE {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMREOF {
    pub emr: EMR,
    pub nPalEntries: u32,
    pub offPalEntries: u32,
    pub nSizeLast: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMREXCLUDECLIPRECT {
    pub emr: EMR,
    pub rclClip: super::super::Foundation::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMREXTCREATEFONTINDIRECTW {
    pub emr: EMR,
    pub ihFont: u32,
    pub elfw: EXTLOGFONTW,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMREXTCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
    pub elp: EXTLOGPEN32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMREXTESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl Default for EMREXTESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMREXTFLOODFILL {
    pub emr: EMR,
    pub ptlStart: super::super::Foundation::POINTL,
    pub crColor: super::super::Foundation::COLORREF,
    pub iMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMREXTSELECTCLIPRGN {
    pub emr: EMR,
    pub cbRgnData: u32,
    pub iMode: u32,
    pub RgnData: [u8; 1],
}
impl Default for EMREXTSELECTCLIPRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMREXTTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub emrtext: EMRTEXT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRFILLPATH {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRFILLRGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub RgnData: [u8; 1],
}
impl Default for EMRFILLRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRFORMAT {
    pub dSignature: u32,
    pub nVersion: u32,
    pub cbData: u32,
    pub offData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRFRAMERGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub szlStroke: super::super::Foundation::SIZE,
    pub RgnData: [u8; 1],
}
impl Default for EMRFRAMERGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRGDICOMMENT {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRGDICOMMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRGLSBOUNDEDRECORD {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRGLSBOUNDEDRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRGLSRECORD {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRGLSRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRGRADIENTFILL {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nVer: u32,
    pub nTri: u32,
    pub ulMode: GRADIENT_FILL,
    pub Ver: [TRIVERTEX; 1],
}
impl Default for EMRGRADIENTFILL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRINVERTRGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub RgnData: [u8; 1],
}
impl Default for EMRINVERTRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRLINETO {
    pub emr: EMR,
    pub ptl: super::super::Foundation::POINTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRMASKBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub xMask: i32,
    pub yMask: i32,
    pub iUsageMask: u32,
    pub offBmiMask: u32,
    pub cbBmiMask: u32,
    pub offBitsMask: u32,
    pub cbBitsMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRMODIFYWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
    pub iMode: MODIFY_WORLD_TRANSFORM_MODE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRNAMEDESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbDriver: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl Default for EMRNAMEDESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMROFFSETCLIPRGN {
    pub emr: EMR,
    pub ptlOffset: super::super::Foundation::POINTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPLGBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub aptlDest: [super::super::Foundation::POINTL; 3],
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub xMask: i32,
    pub yMask: i32,
    pub iUsageMask: u32,
    pub offBmiMask: u32,
    pub cbBmiMask: u32,
    pub offBitsMask: u32,
    pub cbBitsMask: u32,
}
impl Default for EMRPLGBLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPOLYDRAW {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cptl: u32,
    pub aptl: [super::super::Foundation::POINTL; 1],
    pub abTypes: [u8; 1],
}
impl Default for EMRPOLYDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPOLYDRAW16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cpts: u32,
    pub apts: [super::super::Foundation::POINTS; 1],
    pub abTypes: [u8; 1],
}
impl Default for EMRPOLYDRAW16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cptl: u32,
    pub aptl: [super::super::Foundation::POINTL; 1],
}
impl Default for EMRPOLYLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cpts: u32,
    pub apts: [super::super::Foundation::POINTS; 1],
}
impl Default for EMRPOLYLINE16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPOLYPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nPolys: u32,
    pub cptl: u32,
    pub aPolyCounts: [u32; 1],
    pub aptl: [super::super::Foundation::POINTL; 1],
}
impl Default for EMRPOLYPOLYLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPOLYPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nPolys: u32,
    pub cpts: u32,
    pub aPolyCounts: [u32; 1],
    pub apts: [super::super::Foundation::POINTS; 1],
}
impl Default for EMRPOLYPOLYLINE16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPOLYTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub cStrings: i32,
    pub aemrtext: [EMRTEXT; 1],
}
impl Default for EMRPOLYTEXTOUTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRRESIZEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub cEntries: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRRESTOREDC {
    pub emr: EMR,
    pub iRelative: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRROUNDRECT {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
    pub szlCorner: super::super::Foundation::SIZE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSCALEVIEWPORTEXTEX {
    pub emr: EMR,
    pub xNum: i32,
    pub xDenom: i32,
    pub yNum: i32,
    pub yDenom: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSELECTCLIPPATH {
    pub emr: EMR,
    pub iMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSELECTOBJECT {
    pub emr: EMR,
    pub ihObject: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSELECTPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETARCDIRECTION {
    pub emr: EMR,
    pub iArcDirection: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETCOLORADJUSTMENT {
    pub emr: EMR,
    pub ColorAdjustment: COLORADJUSTMENT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETCOLORSPACE {
    pub emr: EMR,
    pub ihCS: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETDIBITSTODEVICE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub iStartScan: u32,
    pub cScans: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRSETICMPROFILE {
    pub emr: EMR,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRSETICMPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETMAPPERFLAGS {
    pub emr: EMR,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETMITERLIMIT {
    pub emr: EMR,
    pub eMiterLimit: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRSETPALETTEENTRIES {
    pub emr: EMR,
    pub ihPal: u32,
    pub iStart: u32,
    pub cEntries: u32,
    pub aPalEntries: [PALETTEENTRY; 1],
}
impl Default for EMRSETPALETTEENTRIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETPIXELV {
    pub emr: EMR,
    pub ptlPixel: super::super::Foundation::POINTL,
    pub crColor: super::super::Foundation::COLORREF,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETTEXTCOLOR {
    pub emr: EMR,
    pub crColor: super::super::Foundation::COLORREF,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETVIEWPORTEXTEX {
    pub emr: EMR,
    pub szlExtent: super::super::Foundation::SIZE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETVIEWPORTORGEX {
    pub emr: EMR,
    pub ptlOrigin: super::super::Foundation::POINTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSETWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSTRETCHBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRSTRETCHDIBITS {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub dwRop: u32,
    pub cxDest: i32,
    pub cyDest: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRTEXT {
    pub ptlReference: super::super::Foundation::POINTL,
    pub nChars: u32,
    pub offString: u32,
    pub fOptions: u32,
    pub rcl: super::super::Foundation::RECTL,
    pub offDx: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EMRTRANSPARENTBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::super::Foundation::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
pub const EMR_ABORTPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(68u32);
pub const EMR_ALPHABLEND: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(114u32);
pub const EMR_ANGLEARC: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(41u32);
pub const EMR_ARC: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(45u32);
pub const EMR_ARCTO: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(55u32);
pub const EMR_BEGINPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(59u32);
pub const EMR_BITBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(76u32);
pub const EMR_CHORD: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(46u32);
pub const EMR_CLOSEFIGURE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(61u32);
pub const EMR_COLORCORRECTPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(111u32);
pub const EMR_COLORMATCHTOTARGETW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(121u32);
pub const EMR_CREATEBRUSHINDIRECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(39u32);
pub const EMR_CREATECOLORSPACE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(99u32);
pub const EMR_CREATECOLORSPACEW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(122u32);
pub const EMR_CREATEDIBPATTERNBRUSHPT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(94u32);
pub const EMR_CREATEMONOBRUSH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(93u32);
pub const EMR_CREATEPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(49u32);
pub const EMR_CREATEPEN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(38u32);
pub const EMR_DELETECOLORSPACE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(101u32);
pub const EMR_DELETEOBJECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(40u32);
pub const EMR_ELLIPSE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(42u32);
pub const EMR_ENDPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(60u32);
pub const EMR_EOF: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(14u32);
pub const EMR_EXCLUDECLIPRECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(29u32);
pub const EMR_EXTCREATEFONTINDIRECTW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(82u32);
pub const EMR_EXTCREATEPEN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(95u32);
pub const EMR_EXTFLOODFILL: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(53u32);
pub const EMR_EXTSELECTCLIPRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(75u32);
pub const EMR_EXTTEXTOUTA: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(83u32);
pub const EMR_EXTTEXTOUTW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(84u32);
pub const EMR_FILLPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(62u32);
pub const EMR_FILLRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(71u32);
pub const EMR_FLATTENPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(65u32);
pub const EMR_FRAMERGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(72u32);
pub const EMR_GDICOMMENT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(70u32);
pub const EMR_GLSBOUNDEDRECORD: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(103u32);
pub const EMR_GLSRECORD: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(102u32);
pub const EMR_GRADIENTFILL: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(118u32);
pub const EMR_HEADER: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(1u32);
pub const EMR_INTERSECTCLIPRECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(30u32);
pub const EMR_INVERTRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(73u32);
pub const EMR_LINETO: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(54u32);
pub const EMR_MASKBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(78u32);
pub const EMR_MAX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(122u32);
pub const EMR_MIN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(1u32);
pub const EMR_MODIFYWORLDTRANSFORM: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(36u32);
pub const EMR_MOVETOEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(27u32);
pub const EMR_OFFSETCLIPRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(26u32);
pub const EMR_PAINTRGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(74u32);
pub const EMR_PIE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(47u32);
pub const EMR_PIXELFORMAT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(104u32);
pub const EMR_PLGBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(79u32);
pub const EMR_POLYBEZIER: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(2u32);
pub const EMR_POLYBEZIER16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(85u32);
pub const EMR_POLYBEZIERTO: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(5u32);
pub const EMR_POLYBEZIERTO16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(88u32);
pub const EMR_POLYDRAW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(56u32);
pub const EMR_POLYDRAW16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(92u32);
pub const EMR_POLYGON: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(3u32);
pub const EMR_POLYGON16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(86u32);
pub const EMR_POLYLINE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(4u32);
pub const EMR_POLYLINE16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(87u32);
pub const EMR_POLYLINETO: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(6u32);
pub const EMR_POLYLINETO16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(89u32);
pub const EMR_POLYPOLYGON: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(8u32);
pub const EMR_POLYPOLYGON16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(91u32);
pub const EMR_POLYPOLYLINE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(7u32);
pub const EMR_POLYPOLYLINE16: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(90u32);
pub const EMR_POLYTEXTOUTA: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(96u32);
pub const EMR_POLYTEXTOUTW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(97u32);
pub const EMR_REALIZEPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(52u32);
pub const EMR_RECTANGLE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(43u32);
pub const EMR_RESERVED_105: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(105u32);
pub const EMR_RESERVED_106: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(106u32);
pub const EMR_RESERVED_107: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(107u32);
pub const EMR_RESERVED_108: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(108u32);
pub const EMR_RESERVED_109: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(109u32);
pub const EMR_RESERVED_110: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(110u32);
pub const EMR_RESERVED_117: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(117u32);
pub const EMR_RESERVED_119: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(119u32);
pub const EMR_RESERVED_120: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(120u32);
pub const EMR_RESIZEPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(51u32);
pub const EMR_RESTOREDC: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(34u32);
pub const EMR_ROUNDRECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(44u32);
pub const EMR_SAVEDC: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(33u32);
pub const EMR_SCALEVIEWPORTEXTEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(31u32);
pub const EMR_SCALEWINDOWEXTEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(32u32);
pub const EMR_SELECTCLIPPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(67u32);
pub const EMR_SELECTOBJECT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(37u32);
pub const EMR_SELECTPALETTE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(48u32);
pub const EMR_SETARCDIRECTION: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(57u32);
pub const EMR_SETBKCOLOR: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(25u32);
pub const EMR_SETBKMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(18u32);
pub const EMR_SETBRUSHORGEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(13u32);
pub const EMR_SETCOLORADJUSTMENT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(23u32);
pub const EMR_SETCOLORSPACE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(100u32);
pub const EMR_SETDIBITSTODEVICE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(80u32);
pub const EMR_SETICMMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(98u32);
pub const EMR_SETICMPROFILEA: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(112u32);
pub const EMR_SETICMPROFILEW: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(113u32);
pub const EMR_SETLAYOUT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(115u32);
pub const EMR_SETMAPMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(17u32);
pub const EMR_SETMAPPERFLAGS: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(16u32);
pub const EMR_SETMETARGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(28u32);
pub const EMR_SETMITERLIMIT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(58u32);
pub const EMR_SETPALETTEENTRIES: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(50u32);
pub const EMR_SETPIXELV: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(15u32);
pub const EMR_SETPOLYFILLMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(19u32);
pub const EMR_SETROP2: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(20u32);
pub const EMR_SETSTRETCHBLTMODE: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(21u32);
pub const EMR_SETTEXTALIGN: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(22u32);
pub const EMR_SETTEXTCOLOR: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(24u32);
pub const EMR_SETVIEWPORTEXTEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(11u32);
pub const EMR_SETVIEWPORTORGEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(12u32);
pub const EMR_SETWINDOWEXTEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(9u32);
pub const EMR_SETWINDOWORGEX: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(10u32);
pub const EMR_SETWORLDTRANSFORM: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(35u32);
pub const EMR_STRETCHBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(77u32);
pub const EMR_STRETCHDIBITS: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(81u32);
pub const EMR_STROKEANDFILLPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(63u32);
pub const EMR_STROKEPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(64u32);
pub const EMR_TRANSPARENTBLT: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(116u32);
pub const EMR_WIDENPATH: ENHANCED_METAFILE_RECORD_TYPE = ENHANCED_METAFILE_RECORD_TYPE(66u32);
pub const ENABLEDUPLEX: u32 = 28u32;
pub const ENABLEPAIRKERNING: u32 = 769u32;
pub const ENABLERELATIVEWIDTHS: u32 = 768u32;
pub const ENCAPSULATED_POSTSCRIPT: u32 = 4116u32;
pub const ENDDOC: u32 = 11u32;
pub const END_PATH: u32 = 4098u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENHANCED_METAFILE_RECORD_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENHMETAHEADER {
    pub iType: u32,
    pub nSize: u32,
    pub rclBounds: super::super::Foundation::RECTL,
    pub rclFrame: super::super::Foundation::RECTL,
    pub dSignature: u32,
    pub nVersion: u32,
    pub nBytes: u32,
    pub nRecords: u32,
    pub nHandles: u16,
    pub sReserved: u16,
    pub nDescription: u32,
    pub offDescription: u32,
    pub nPalEntries: u32,
    pub szlDevice: super::super::Foundation::SIZE,
    pub szlMillimeters: super::super::Foundation::SIZE,
    pub cbPixelFormat: u32,
    pub offPixelFormat: u32,
    pub bOpenGL: u32,
    pub szlMicrometers: super::super::Foundation::SIZE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENHMETARECORD {
    pub iType: ENHANCED_METAFILE_RECORD_TYPE,
    pub nSize: u32,
    pub dParm: [u32; 1],
}
impl Default for ENHMETARECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENHMETA_SIGNATURE: u32 = 1179469088u32;
pub const ENHMETA_STOCK_OBJECT: u32 = 2147483648u32;
pub type ENHMFENUMPROC = Option<unsafe extern "system" fn(hdc: HDC, lpht: *const HANDLETABLE, lpmr: *const ENHMETARECORD, nhandles: i32, data: super::super::Foundation::LPARAM) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
}
impl Default for ENUMLOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMLOGFONTEXA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfScript: [u8; 32],
}
impl Default for ENUMLOGFONTEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUMLOGFONTEXDVA {
    pub elfEnumLogfontEx: ENUMLOGFONTEXA,
    pub elfDesignVector: DESIGNVECTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUMLOGFONTEXDVW {
    pub elfEnumLogfontEx: ENUMLOGFONTEXW,
    pub elfDesignVector: DESIGNVECTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMLOGFONTEXW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfScript: [u16; 32],
}
impl Default for ENUMLOGFONTEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
}
impl Default for ENUMLOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENUMPAPERBINS: u32 = 31u32;
pub const ENUMPAPERMETRICS: u32 = 34u32;
pub const ENUM_CURRENT_SETTINGS: ENUM_DISPLAY_SETTINGS_MODE = ENUM_DISPLAY_SETTINGS_MODE(4294967295u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENUM_DISPLAY_SETTINGS_FLAGS(pub u32);
impl ENUM_DISPLAY_SETTINGS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ENUM_DISPLAY_SETTINGS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ENUM_DISPLAY_SETTINGS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ENUM_DISPLAY_SETTINGS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ENUM_DISPLAY_SETTINGS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ENUM_DISPLAY_SETTINGS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENUM_DISPLAY_SETTINGS_MODE(pub u32);
pub const ENUM_REGISTRY_SETTINGS: ENUM_DISPLAY_SETTINGS_MODE = ENUM_DISPLAY_SETTINGS_MODE(4294967294u32);
pub const EPSPRINTING: u32 = 33u32;
pub const EPS_SIGNATURE: u32 = 1179865157u32;
pub const ERROR: i32 = 0i32;
pub const ERR_FORMAT: u32 = 1006u32;
pub const ERR_GENERIC: u32 = 1000u32;
pub const ERR_INVALID_BASE: u32 = 1085u32;
pub const ERR_INVALID_CMAP: u32 = 1060u32;
pub const ERR_INVALID_DELTA_FORMAT: u32 = 1013u32;
pub const ERR_INVALID_EBLC: u32 = 1086u32;
pub const ERR_INVALID_GDEF: u32 = 1083u32;
pub const ERR_INVALID_GLYF: u32 = 1061u32;
pub const ERR_INVALID_GPOS: u32 = 1082u32;
pub const ERR_INVALID_GSUB: u32 = 1081u32;
pub const ERR_INVALID_HDMX: u32 = 1089u32;
pub const ERR_INVALID_HEAD: u32 = 1062u32;
pub const ERR_INVALID_HHEA: u32 = 1063u32;
pub const ERR_INVALID_HHEA_OR_VHEA: u32 = 1072u32;
pub const ERR_INVALID_HMTX: u32 = 1064u32;
pub const ERR_INVALID_HMTX_OR_VMTX: u32 = 1073u32;
pub const ERR_INVALID_JSTF: u32 = 1084u32;
pub const ERR_INVALID_LOCA: u32 = 1065u32;
pub const ERR_INVALID_LTSH: u32 = 1087u32;
pub const ERR_INVALID_MAXP: u32 = 1066u32;
pub const ERR_INVALID_MERGE_CHECKSUMS: u32 = 1011u32;
pub const ERR_INVALID_MERGE_FORMATS: u32 = 1010u32;
pub const ERR_INVALID_MERGE_NUMGLYPHS: u32 = 1012u32;
pub const ERR_INVALID_NAME: u32 = 1067u32;
pub const ERR_INVALID_OS2: u32 = 1069u32;
pub const ERR_INVALID_POST: u32 = 1068u32;
pub const ERR_INVALID_TTC_INDEX: u32 = 1015u32;
pub const ERR_INVALID_TTO: u32 = 1080u32;
pub const ERR_INVALID_VDMX: u32 = 1088u32;
pub const ERR_INVALID_VHEA: u32 = 1070u32;
pub const ERR_INVALID_VMTX: u32 = 1071u32;
pub const ERR_MEM: u32 = 1005u32;
pub const ERR_MISSING_CMAP: u32 = 1030u32;
pub const ERR_MISSING_EBDT: u32 = 1044u32;
pub const ERR_MISSING_GLYF: u32 = 1031u32;
pub const ERR_MISSING_HEAD: u32 = 1032u32;
pub const ERR_MISSING_HHEA: u32 = 1033u32;
pub const ERR_MISSING_HHEA_OR_VHEA: u32 = 1042u32;
pub const ERR_MISSING_HMTX: u32 = 1034u32;
pub const ERR_MISSING_HMTX_OR_VMTX: u32 = 1043u32;
pub const ERR_MISSING_LOCA: u32 = 1035u32;
pub const ERR_MISSING_MAXP: u32 = 1036u32;
pub const ERR_MISSING_NAME: u32 = 1037u32;
pub const ERR_MISSING_OS2: u32 = 1039u32;
pub const ERR_MISSING_POST: u32 = 1038u32;
pub const ERR_MISSING_VHEA: u32 = 1040u32;
pub const ERR_MISSING_VMTX: u32 = 1041u32;
pub const ERR_NOT_TTC: u32 = 1014u32;
pub const ERR_NO_GLYPHS: u32 = 1009u32;
pub const ERR_PARAMETER0: u32 = 1100u32;
pub const ERR_PARAMETER1: u32 = 1101u32;
pub const ERR_PARAMETER10: u32 = 1110u32;
pub const ERR_PARAMETER11: u32 = 1111u32;
pub const ERR_PARAMETER12: u32 = 1112u32;
pub const ERR_PARAMETER13: u32 = 1113u32;
pub const ERR_PARAMETER14: u32 = 1114u32;
pub const ERR_PARAMETER15: u32 = 1115u32;
pub const ERR_PARAMETER16: u32 = 1116u32;
pub const ERR_PARAMETER2: u32 = 1102u32;
pub const ERR_PARAMETER3: u32 = 1103u32;
pub const ERR_PARAMETER4: u32 = 1104u32;
pub const ERR_PARAMETER5: u32 = 1105u32;
pub const ERR_PARAMETER6: u32 = 1106u32;
pub const ERR_PARAMETER7: u32 = 1107u32;
pub const ERR_PARAMETER8: u32 = 1108u32;
pub const ERR_PARAMETER9: u32 = 1109u32;
pub const ERR_READCONTROL: u32 = 1003u32;
pub const ERR_READOUTOFBOUNDS: u32 = 1001u32;
pub const ERR_VERSION: u32 = 1008u32;
pub const ERR_WOULD_GROW: u32 = 1007u32;
pub const ERR_WRITECONTROL: u32 = 1004u32;
pub const ERR_WRITEOUTOFBOUNDS: u32 = 1002u32;
pub const ETO_CLIPPED: ETO_OPTIONS = ETO_OPTIONS(4u32);
pub const ETO_GLYPH_INDEX: ETO_OPTIONS = ETO_OPTIONS(16u32);
pub const ETO_IGNORELANGUAGE: ETO_OPTIONS = ETO_OPTIONS(4096u32);
pub const ETO_NUMERICSLATIN: ETO_OPTIONS = ETO_OPTIONS(2048u32);
pub const ETO_NUMERICSLOCAL: ETO_OPTIONS = ETO_OPTIONS(1024u32);
pub const ETO_OPAQUE: ETO_OPTIONS = ETO_OPTIONS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETO_OPTIONS(pub u32);
impl ETO_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ETO_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ETO_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ETO_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ETO_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ETO_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ETO_PDY: ETO_OPTIONS = ETO_OPTIONS(8192u32);
pub const ETO_REVERSE_INDEX_MAP: ETO_OPTIONS = ETO_OPTIONS(65536u32);
pub const ETO_RTLREADING: ETO_OPTIONS = ETO_OPTIONS(128u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
impl Default for EXTLOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
impl Default for EXTLOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTLOGPEN {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: super::super::Foundation::COLORREF,
    pub elpHatch: usize,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
impl Default for EXTLOGPEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTLOGPEN32 {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: super::super::Foundation::COLORREF,
    pub elpHatch: u32,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
impl Default for EXTLOGPEN32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXTTEXTOUT: u32 = 512u32;
pub const EXT_DEVICE_CAPS: u32 = 4099u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXT_FLOOD_FILL_TYPE(pub u32);
pub const E_ADDFONTFAILED: i32 = 512i32;
pub const E_API_NOTIMPL: i32 = 1i32;
pub const E_CHARCODECOUNTINVALID: i32 = 2i32;
pub const E_CHARCODESETINVALID: i32 = 3i32;
pub const E_CHARSETINVALID: i32 = 21i32;
pub const E_COULDNTCREATETEMPFILE: i32 = 513i32;
pub const E_DEVICETRUETYPEFONT: i32 = 4i32;
pub const E_ERRORACCESSINGEXCLUDELIST: i32 = 274i32;
pub const E_ERRORACCESSINGFACENAME: i32 = 13i32;
pub const E_ERRORACCESSINGFONTDATA: i32 = 12i32;
pub const E_ERRORCOMPRESSINGFONTDATA: i32 = 256i32;
pub const E_ERRORCONVERTINGCHARS: i32 = 18i32;
pub const E_ERRORCREATINGFONTFILE: i32 = 269i32;
pub const E_ERRORDECOMPRESSINGFONTDATA: i32 = 273i32;
pub const E_ERROREXPANDINGFONTDATA: i32 = 519i32;
pub const E_ERRORGETTINGDC: i32 = 520i32;
pub const E_ERRORREADINGFONTDATA: i32 = 267i32;
pub const E_ERRORUNICODECONVERSION: i32 = 17i32;
pub const E_EXCEPTION: i32 = 19i32;
pub const E_EXCEPTIONINCOMPRESSION: i32 = 522i32;
pub const E_EXCEPTIONINDECOMPRESSION: i32 = 521i32;
pub const E_FACENAMEINVALID: i32 = 275i32;
pub const E_FILE_NOT_FOUND: i32 = 23i32;
pub const E_FLAGSINVALID: i32 = 268i32;
pub const E_FONTALREADYEXISTS: i32 = 270i32;
pub const E_FONTDATAINVALID: i32 = 258i32;
pub const E_FONTFAMILYNAMENOTINFULL: i32 = 285i32;
pub const E_FONTFILECREATEFAILED: i32 = 515i32;
pub const E_FONTFILENOTFOUND: i32 = 517i32;
pub const E_FONTINSTALLFAILED: i32 = 272i32;
pub const E_FONTNAMEALREADYEXISTS: i32 = 271i32;
pub const E_FONTNOTEMBEDDABLE: i32 = 260i32;
pub const E_FONTREFERENCEINVALID: i32 = 8i32;
pub const E_FONTVARIATIONSIMULATED: i32 = 283i32;
pub const E_HDCINVALID: i32 = 6i32;
pub const E_INPUTPARAMINVALID: i32 = 25i32;
pub const E_NAMECHANGEFAILED: i32 = 259i32;
pub const E_NOFREEMEMORY: i32 = 7i32;
pub const E_NONE: i32 = 0i32;
pub const E_NOOS2: i32 = 265i32;
pub const E_NOTATRUETYPEFONT: i32 = 10i32;
pub const E_PBENABLEDINVALID: i32 = 280i32;
pub const E_PERMISSIONSINVALID: i32 = 279i32;
pub const E_PRIVSINVALID: i32 = 261i32;
pub const E_PRIVSTATUSINVALID: i32 = 278i32;
pub const E_READFROMSTREAMFAILED: i32 = 263i32;
pub const E_RESERVEDPARAMNOTNULL: i32 = 20i32;
pub const E_RESOURCEFILECREATEFAILED: i32 = 518i32;
pub const E_SAVETOSTREAMFAILED: i32 = 264i32;
pub const E_STATUSINVALID: i32 = 277i32;
pub const E_STREAMINVALID: i32 = 276i32;
pub const E_SUBSETTINGEXCEPTION: i32 = 281i32;
pub const E_SUBSETTINGFAILED: i32 = 262i32;
pub const E_SUBSTRING_TEST_FAIL: i32 = 282i32;
pub const E_T2NOFREEMEMORY: i32 = 266i32;
pub const E_TTC_INDEX_OUT_OF_RANGE: i32 = 24i32;
pub const E_WINDOWSAPI: i32 = 516i32;
pub const FEATURESETTING_CUSTPAPER: u32 = 3u32;
pub const FEATURESETTING_MIRROR: u32 = 4u32;
pub const FEATURESETTING_NEGATIVE: u32 = 5u32;
pub const FEATURESETTING_NUP: u32 = 0u32;
pub const FEATURESETTING_OUTPUT: u32 = 1u32;
pub const FEATURESETTING_PRIVATE_BEGIN: u32 = 4096u32;
pub const FEATURESETTING_PRIVATE_END: u32 = 8191u32;
pub const FEATURESETTING_PROTOCOL: u32 = 6u32;
pub const FEATURESETTING_PSLEVEL: u32 = 2u32;
pub const FF_DECORATIVE: FONT_FAMILY = FONT_FAMILY(80u8);
pub const FF_DONTCARE: FONT_FAMILY = FONT_FAMILY(0u8);
pub const FF_MODERN: FONT_FAMILY = FONT_FAMILY(48u8);
pub const FF_ROMAN: FONT_FAMILY = FONT_FAMILY(16u8);
pub const FF_SCRIPT: FONT_FAMILY = FONT_FAMILY(64u8);
pub const FF_SWISS: FONT_FAMILY = FONT_FAMILY(32u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FIXED {
    pub fract: u16,
    pub value: i16,
}
pub const FIXED_PITCH: FONT_PITCH = FONT_PITCH(1u8);
pub const FLI_GLYPHS: i32 = 262144i32;
pub const FLI_MASK: u32 = 4155u32;
pub const FLOODFILLBORDER: EXT_FLOOD_FILL_TYPE = EXT_FLOOD_FILL_TYPE(0u32);
pub const FLOODFILLSURFACE: EXT_FLOOD_FILL_TYPE = EXT_FLOOD_FILL_TYPE(1u32);
pub const FLUSHOUTPUT: u32 = 6u32;
pub type FONTENUMPROCA = Option<unsafe extern "system" fn(param0: *const LOGFONTA, param1: *const TEXTMETRICA, param2: u32, param3: super::super::Foundation::LPARAM) -> i32>;
pub type FONTENUMPROCW = Option<unsafe extern "system" fn(param0: *const LOGFONTW, param1: *const TEXTMETRICW, param2: u32, param3: super::super::Foundation::LPARAM) -> i32>;
pub const FONTMAPPER_MAX: u32 = 10u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_CHARSET(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_CLIP_PRECISION(pub u8);
impl FONT_CLIP_PRECISION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FONT_CLIP_PRECISION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FONT_CLIP_PRECISION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FONT_CLIP_PRECISION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FONT_CLIP_PRECISION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FONT_CLIP_PRECISION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_FAMILY(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_LICENSE_PRIVS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_OUTPUT_PRECISION(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_PITCH(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_QUALITY(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_RESOURCE_CHARACTERISTICS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FONT_WEIGHT(pub u32);
pub const FR_NOT_ENUM: FONT_RESOURCE_CHARACTERISTICS = FONT_RESOURCE_CHARACTERISTICS(32u32);
pub const FR_PRIVATE: FONT_RESOURCE_CHARACTERISTICS = FONT_RESOURCE_CHARACTERISTICS(16u32);
pub const FS_ARABIC: i32 = 64i32;
pub const FS_BALTIC: i32 = 128i32;
pub const FS_CHINESESIMP: i32 = 262144i32;
pub const FS_CHINESETRAD: i32 = 1048576i32;
pub const FS_CYRILLIC: i32 = 4i32;
pub const FS_GREEK: i32 = 8i32;
pub const FS_HEBREW: i32 = 32i32;
pub const FS_JISJAPAN: i32 = 131072i32;
pub const FS_JOHAB: i32 = 2097152i32;
pub const FS_LATIN1: i32 = 1i32;
pub const FS_LATIN2: i32 = 2i32;
pub const FS_SYMBOL: i32 = -2147483648i32;
pub const FS_THAI: i32 = 65536i32;
pub const FS_TURKISH: i32 = 16i32;
pub const FS_VIETNAMESE: i32 = 256i32;
pub const FS_WANSUNG: i32 = 524288i32;
pub const FW_BLACK: FONT_WEIGHT = FONT_WEIGHT(900u32);
pub const FW_BOLD: FONT_WEIGHT = FONT_WEIGHT(700u32);
pub const FW_DEMIBOLD: FONT_WEIGHT = FONT_WEIGHT(600u32);
pub const FW_DONTCARE: FONT_WEIGHT = FONT_WEIGHT(0u32);
pub const FW_EXTRABOLD: FONT_WEIGHT = FONT_WEIGHT(800u32);
pub const FW_EXTRALIGHT: FONT_WEIGHT = FONT_WEIGHT(200u32);
pub const FW_HEAVY: FONT_WEIGHT = FONT_WEIGHT(900u32);
pub const FW_LIGHT: FONT_WEIGHT = FONT_WEIGHT(300u32);
pub const FW_MEDIUM: FONT_WEIGHT = FONT_WEIGHT(500u32);
pub const FW_NORMAL: FONT_WEIGHT = FONT_WEIGHT(400u32);
pub const FW_REGULAR: FONT_WEIGHT = FONT_WEIGHT(400u32);
pub const FW_SEMIBOLD: FONT_WEIGHT = FONT_WEIGHT(600u32);
pub const FW_THIN: FONT_WEIGHT = FONT_WEIGHT(100u32);
pub const FW_ULTRABOLD: FONT_WEIGHT = FONT_WEIGHT(800u32);
pub const FW_ULTRALIGHT: FONT_WEIGHT = FONT_WEIGHT(200u32);
pub const GB2312_CHARSET: FONT_CHARSET = FONT_CHARSET(134u8);
pub const GCPCLASS_ARABIC: u32 = 2u32;
pub const GCPCLASS_HEBREW: u32 = 2u32;
pub const GCPCLASS_LATIN: u32 = 1u32;
pub const GCPCLASS_LATINNUMBER: u32 = 5u32;
pub const GCPCLASS_LATINNUMERICSEPARATOR: u32 = 7u32;
pub const GCPCLASS_LATINNUMERICTERMINATOR: u32 = 6u32;
pub const GCPCLASS_LOCALNUMBER: u32 = 4u32;
pub const GCPCLASS_NEUTRAL: u32 = 3u32;
pub const GCPCLASS_NUMERICSEPARATOR: u32 = 8u32;
pub const GCPCLASS_POSTBOUNDLTR: u32 = 32u32;
pub const GCPCLASS_POSTBOUNDRTL: u32 = 16u32;
pub const GCPCLASS_PREBOUNDLTR: u32 = 128u32;
pub const GCPCLASS_PREBOUNDRTL: u32 = 64u32;
pub const GCPGLYPH_LINKAFTER: u32 = 16384u32;
pub const GCPGLYPH_LINKBEFORE: u32 = 32768u32;
pub const GCP_CLASSIN: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(524288u32);
pub const GCP_DBCS: u32 = 1u32;
pub const GCP_DIACRITIC: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(256u32);
pub const GCP_DISPLAYZWG: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(4194304u32);
pub const GCP_ERROR: u32 = 32768u32;
pub const GCP_GLYPHSHAPE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(16u32);
pub const GCP_JUSTIFY: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(65536u32);
pub const GCP_JUSTIFYIN: i32 = 2097152i32;
pub const GCP_KASHIDA: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(1024u32);
pub const GCP_LIGATE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(32u32);
pub const GCP_MAXEXTENT: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(1048576u32);
pub const GCP_NEUTRALOVERRIDE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(33554432u32);
pub const GCP_NUMERICOVERRIDE: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(16777216u32);
pub const GCP_NUMERICSLATIN: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(67108864u32);
pub const GCP_NUMERICSLOCAL: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(134217728u32);
pub const GCP_REORDER: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCP_RESULTSA {
    pub lStructSize: u32,
    pub lpOutString: windows_core::PSTR,
    pub lpOrder: *mut u32,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: windows_core::PSTR,
    pub lpGlyphs: windows_core::PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl Default for GCP_RESULTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCP_RESULTSW {
    pub lStructSize: u32,
    pub lpOutString: windows_core::PWSTR,
    pub lpOrder: *mut u32,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: windows_core::PSTR,
    pub lpGlyphs: windows_core::PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl Default for GCP_RESULTSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GCP_SYMSWAPOFF: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(8388608u32);
pub const GCP_USEKERNING: GET_CHARACTER_PLACEMENT_FLAGS = GET_CHARACTER_PLACEMENT_FLAGS(8u32);
pub const GDICOMMENT_BEGINGROUP: u32 = 2u32;
pub const GDICOMMENT_ENDGROUP: u32 = 3u32;
pub const GDICOMMENT_IDENTIFIER: u32 = 1128875079u32;
pub const GDICOMMENT_MULTIFORMATS: u32 = 1073741828u32;
pub const GDICOMMENT_UNICODE_END: u32 = 128u32;
pub const GDICOMMENT_UNICODE_STRING: u32 = 64u32;
pub const GDICOMMENT_WINDOWS_METAFILE: u32 = 2147483649u32;
pub const GDIPLUS_TS_QUERYVER: u32 = 4122u32;
pub const GDIPLUS_TS_RECORD: u32 = 4123u32;
pub const GDIREGISTERDDRAWPACKETVERSION: u32 = 1u32;
pub const GDI_ERROR: i32 = -1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GDI_REGION_TYPE(pub i32);
pub const GETCOLORTABLE: u32 = 5u32;
pub const GETDEVICEUNITS: u32 = 42u32;
pub const GETEXTENDEDTEXTMETRICS: u32 = 256u32;
pub const GETEXTENTTABLE: u32 = 257u32;
pub const GETFACENAME: u32 = 513u32;
pub const GETPAIRKERNTABLE: u32 = 258u32;
pub const GETPENWIDTH: u32 = 16u32;
pub const GETPHYSPAGESIZE: u32 = 12u32;
pub const GETPRINTINGOFFSET: u32 = 13u32;
pub const GETSCALINGFACTOR: u32 = 14u32;
pub const GETSETPAPERBINS: u32 = 29u32;
pub const GETSETPAPERMETRICS: u32 = 35u32;
pub const GETSETPRINTORIENT: u32 = 30u32;
pub const GETSETSCREENPARAMS: u32 = 3072u32;
pub const GETTECHNOLGY: u32 = 20u32;
pub const GETTECHNOLOGY: u32 = 20u32;
pub const GETTRACKKERNTABLE: u32 = 259u32;
pub const GETVECTORBRUSHSIZE: u32 = 27u32;
pub const GETVECTORPENSIZE: u32 = 26u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_CHARACTER_PLACEMENT_FLAGS(pub u32);
impl GET_CHARACTER_PLACEMENT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GET_CHARACTER_PLACEMENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GET_CHARACTER_PLACEMENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_DCX_FLAGS(pub u32);
impl GET_DCX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GET_DCX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GET_DCX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GET_DCX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GET_DCX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GET_DCX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_DEVICE_CAPS_INDEX(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_GLYPH_OUTLINE_FORMAT(pub u32);
pub const GET_PS_FEATURESETTING: u32 = 4121u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_STOCK_OBJECT_FLAGS(pub i32);
pub const GGI_MARK_NONEXISTING_GLYPHS: u32 = 1u32;
pub const GGO_BEZIER: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(3u32);
pub const GGO_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(1u32);
pub const GGO_GLYPH_INDEX: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(128u32);
pub const GGO_GRAY2_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(4u32);
pub const GGO_GRAY4_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(5u32);
pub const GGO_GRAY8_BITMAP: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(6u32);
pub const GGO_METRICS: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(0u32);
pub const GGO_NATIVE: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(2u32);
pub const GGO_UNHINTED: GET_GLYPH_OUTLINE_FORMAT = GET_GLYPH_OUTLINE_FORMAT(256u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLYPHMETRICS {
    pub gmBlackBoxX: u32,
    pub gmBlackBoxY: u32,
    pub gmptGlyphOrigin: super::super::Foundation::POINT,
    pub gmCellIncX: i16,
    pub gmCellIncY: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GLYPHSET {
    pub cbThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRanges: u32,
    pub ranges: [WCRANGE; 1],
}
impl Default for GLYPHSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GM_ADVANCED: GRAPHICS_MODE = GRAPHICS_MODE(2i32);
pub const GM_COMPATIBLE: GRAPHICS_MODE = GRAPHICS_MODE(1i32);
pub const GM_LAST: u32 = 2u32;
pub type GOBJENUMPROC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::super::Foundation::LPARAM) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRADIENT_FILL(pub u32);
pub const GRADIENT_FILL_OP_FLAG: u32 = 255u32;
pub const GRADIENT_FILL_RECT_H: GRADIENT_FILL = GRADIENT_FILL(0u32);
pub const GRADIENT_FILL_RECT_V: GRADIENT_FILL = GRADIENT_FILL(1u32);
pub const GRADIENT_FILL_TRIANGLE: GRADIENT_FILL = GRADIENT_FILL(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GRADIENT_RECT {
    pub UpperLeft: u32,
    pub LowerRight: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GRADIENT_TRIANGLE {
    pub Vertex1: u32,
    pub Vertex2: u32,
    pub Vertex3: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRAPHICS_MODE(pub i32);
pub type GRAYSTRINGPROC = Option<unsafe extern "system" fn(param0: HDC, param1: super::super::Foundation::LPARAM, param2: i32) -> windows_core::BOOL>;
pub const GRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(2i32);
pub const GREEK_CHARSET: FONT_CHARSET = FONT_CHARSET(161u8);
pub const GS_8BIT_INDICES: u32 = 1u32;
pub const HALFTONE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HANDLETABLE {
    pub objectHandle: [HGDIOBJ; 1],
}
impl Default for HANDLETABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HANGEUL_CHARSET: FONT_CHARSET = FONT_CHARSET(129u8);
pub const HANGUL_CHARSET: FONT_CHARSET = FONT_CHARSET(129u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HATCH_BRUSH_STYLE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HBITMAP(pub *mut core::ffi::c_void);
impl HBITMAP {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HBITMAP {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}
impl Default for HBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<HGDIOBJ> for HBITMAP {}
impl From<HBITMAP> for HGDIOBJ {
    fn from(value: HBITMAP) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HBRUSH(pub *mut core::ffi::c_void);
impl HBRUSH {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HBRUSH {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}
impl Default for HBRUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<HGDIOBJ> for HBRUSH {}
impl From<HBRUSH> for HGDIOBJ {
    fn from(value: HBRUSH) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDC(pub *mut core::ffi::c_void);
impl HDC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HDC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HDC_MAP_MODE(pub i32);
pub const HEBREW_CHARSET: FONT_CHARSET = FONT_CHARSET(177u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HENHMETAFILE(pub *mut core::ffi::c_void);
impl HENHMETAFILE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HENHMETAFILE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteEnhMetaFile(hmf : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteEnhMetaFile(self.0);
            }
        }
    }
}
impl Default for HENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HFONT(pub *mut core::ffi::c_void);
impl HFONT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HFONT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}
impl Default for HFONT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<HGDIOBJ> for HFONT {}
impl From<HFONT> for HGDIOBJ {
    fn from(value: HFONT) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGDIOBJ(pub *mut core::ffi::c_void);
impl HGDIOBJ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HGDIOBJ {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}
impl Default for HGDIOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMETAFILE(pub *mut core::ffi::c_void);
impl HMETAFILE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HMETAFILE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteMetaFile(hmf : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteMetaFile(self.0);
            }
        }
    }
}
impl Default for HMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMONITOR(pub *mut core::ffi::c_void);
impl HMONITOR {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HMONITOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HOLLOW_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(5i32);
pub const HORZRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(8u32);
pub const HORZSIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPALETTE(pub *mut core::ffi::c_void);
impl HPALETTE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HPALETTE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}
impl Default for HPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<HGDIOBJ> for HPALETTE {}
impl From<HPALETTE> for HGDIOBJ {
    fn from(value: HPALETTE) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPEN(pub *mut core::ffi::c_void);
impl HPEN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HPEN {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}
impl Default for HPEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<HGDIOBJ> for HPEN {}
impl From<HPEN> for HGDIOBJ {
    fn from(value: HPEN) -> Self {
        Self(value.0)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRGN(pub *mut core::ffi::c_void);
impl HRGN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HRGN {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}
impl Default for HRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<HGDIOBJ> for HRGN {}
impl From<HRGN> for HGDIOBJ {
    fn from(value: HRGN) -> Self {
        Self(value.0)
    }
}
pub const HS_API_MAX: u32 = 12u32;
pub const HS_BDIAGONAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(3i32);
pub const HS_CROSS: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(4i32);
pub const HS_DIAGCROSS: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(5i32);
pub const HS_FDIAGONAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(2i32);
pub const HS_HORIZONTAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(0i32);
pub const HS_VERTICAL: HATCH_BRUSH_STYLE = HATCH_BRUSH_STYLE(1i32);
pub const ILLUMINANT_A: u32 = 1u32;
pub const ILLUMINANT_B: u32 = 2u32;
pub const ILLUMINANT_C: u32 = 3u32;
pub const ILLUMINANT_D50: u32 = 4u32;
pub const ILLUMINANT_D55: u32 = 5u32;
pub const ILLUMINANT_D65: u32 = 6u32;
pub const ILLUMINANT_D75: u32 = 7u32;
pub const ILLUMINANT_DAYLIGHT: u32 = 3u32;
pub const ILLUMINANT_DEVICE_DEFAULT: u32 = 0u32;
pub const ILLUMINANT_F2: u32 = 8u32;
pub const ILLUMINANT_FLUORESCENT: u32 = 8u32;
pub const ILLUMINANT_MAX_INDEX: u32 = 8u32;
pub const ILLUMINANT_NTSC: u32 = 3u32;
pub const ILLUMINANT_TUNGSTEN: u32 = 1u32;
pub const JOHAB_CHARSET: FONT_CHARSET = FONT_CHARSET(130u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERNINGPAIR {
    pub wFirst: u16,
    pub wSecond: u16,
    pub iKernAmount: i32,
}
pub const LAYOUT_BITMAPORIENTATIONPRESERVED: DC_LAYOUT = DC_LAYOUT(8u32);
pub const LAYOUT_BTT: u32 = 2u32;
pub const LAYOUT_RTL: DC_LAYOUT = DC_LAYOUT(1u32);
pub const LAYOUT_VBH: u32 = 4u32;
pub const LCS_GM_ABS_COLORIMETRIC: i32 = 8i32;
pub const LCS_GM_BUSINESS: i32 = 1i32;
pub const LCS_GM_GRAPHICS: i32 = 2i32;
pub const LCS_GM_IMAGES: i32 = 4i32;
pub const LC_INTERIORS: u32 = 128u32;
pub const LC_MARKER: u32 = 4u32;
pub const LC_NONE: u32 = 0u32;
pub const LC_POLYLINE: u32 = 2u32;
pub const LC_POLYMARKER: u32 = 8u32;
pub const LC_STYLED: u32 = 32u32;
pub const LC_WIDE: u32 = 16u32;
pub const LC_WIDESTYLED: u32 = 64u32;
pub const LF_FACESIZE: u32 = 32u32;
pub const LF_FULLFACESIZE: u32 = 64u32;
pub const LICENSE_DEFAULT: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(0u32);
pub const LICENSE_EDITABLE: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(8u32);
pub const LICENSE_INSTALLABLE: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(0u32);
pub const LICENSE_NOEMBEDDING: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(2u32);
pub const LICENSE_PREVIEWPRINT: FONT_LICENSE_PRIVS = FONT_LICENSE_PRIVS(4u32);
pub const LINECAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(30u32);
pub type LINEDDAPROC = Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: super::super::Foundation::LPARAM)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LOGBRUSH {
    pub lbStyle: BRUSH_STYLE,
    pub lbColor: super::super::Foundation::COLORREF,
    pub lbHatch: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LOGBRUSH32 {
    pub lbStyle: BRUSH_STYLE,
    pub lbColor: super::super::Foundation::COLORREF,
    pub lbHatch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOGFONTA {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: FONT_CHARSET,
    pub lfOutPrecision: FONT_OUTPUT_PRECISION,
    pub lfClipPrecision: FONT_CLIP_PRECISION,
    pub lfQuality: FONT_QUALITY,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [i8; 32],
}
impl Default for LOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOGFONTW {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: FONT_CHARSET,
    pub lfOutPrecision: FONT_OUTPUT_PRECISION,
    pub lfClipPrecision: FONT_CLIP_PRECISION,
    pub lfQuality: FONT_QUALITY,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [u16; 32],
}
impl Default for LOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOGPALETTE {
    pub palVersion: u16,
    pub palNumEntries: u16,
    pub palPalEntry: [PALETTEENTRY; 1],
}
impl Default for LOGPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LOGPEN {
    pub lopnStyle: u32,
    pub lopnWidth: super::super::Foundation::POINT,
    pub lopnColor: super::super::Foundation::COLORREF,
}
pub const LOGPIXELSX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(88u32);
pub const LOGPIXELSY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(90u32);
pub const LPD_DOUBLEBUFFER: u32 = 1u32;
pub const LPD_SHARE_ACCUM: u32 = 256u32;
pub const LPD_SHARE_DEPTH: u32 = 64u32;
pub const LPD_SHARE_STENCIL: u32 = 128u32;
pub const LPD_STEREO: u32 = 2u32;
pub const LPD_SUPPORT_GDI: u32 = 16u32;
pub const LPD_SUPPORT_OPENGL: u32 = 32u32;
pub const LPD_SWAP_COPY: u32 = 1024u32;
pub const LPD_SWAP_EXCHANGE: u32 = 512u32;
pub const LPD_TRANSPARENT: u32 = 4096u32;
pub const LPD_TYPE_COLORINDEX: u32 = 1u32;
pub const LPD_TYPE_RGBA: u32 = 0u32;
pub type LPFNDEVCAPS = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: u32, param3: windows_core::PCSTR, param4: *mut DEVMODEA) -> u32>;
pub type LPFNDEVMODE = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::HMODULE, param2: *mut DEVMODEA, param3: windows_core::PCSTR, param4: windows_core::PCSTR, param5: *mut DEVMODEA, param6: windows_core::PCSTR, param7: u32) -> u32>;
pub const LTGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(1i32);
pub const MAC_CHARSET: FONT_CHARSET = FONT_CHARSET(77u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MAT2 {
    pub eM11: FIXED,
    pub eM12: FIXED,
    pub eM21: FIXED,
    pub eM22: FIXED,
}
pub const MAXSTRETCHBLTMODE: u32 = 4u32;
pub const MERGECOPY: ROP_CODE = ROP_CODE(12583114u32);
pub const MERGEPAINT: ROP_CODE = ROP_CODE(12255782u32);
pub const METAFILE_DRIVER: u32 = 2049u32;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct METAHEADER {
    pub mtType: u16,
    pub mtHeaderSize: u16,
    pub mtVersion: u16,
    pub mtSize: u32,
    pub mtNoObjects: u16,
    pub mtMaxRecord: u32,
    pub mtNoParameters: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct METARECORD {
    pub rdSize: u32,
    pub rdFunction: u16,
    pub rdParm: [u16; 1],
}
impl Default for METARECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const META_ANIMATEPALETTE: u32 = 1078u32;
pub const META_ARC: u32 = 2071u32;
pub const META_BITBLT: u32 = 2338u32;
pub const META_CHORD: u32 = 2096u32;
pub const META_CREATEBRUSHINDIRECT: u32 = 764u32;
pub const META_CREATEFONTINDIRECT: u32 = 763u32;
pub const META_CREATEPALETTE: u32 = 247u32;
pub const META_CREATEPATTERNBRUSH: u32 = 505u32;
pub const META_CREATEPENINDIRECT: u32 = 762u32;
pub const META_CREATEREGION: u32 = 1791u32;
pub const META_DELETEOBJECT: u32 = 496u32;
pub const META_DIBBITBLT: u32 = 2368u32;
pub const META_DIBCREATEPATTERNBRUSH: u32 = 322u32;
pub const META_DIBSTRETCHBLT: u32 = 2881u32;
pub const META_ELLIPSE: u32 = 1048u32;
pub const META_ESCAPE: u32 = 1574u32;
pub const META_EXCLUDECLIPRECT: u32 = 1045u32;
pub const META_EXTFLOODFILL: u32 = 1352u32;
pub const META_EXTTEXTOUT: u32 = 2610u32;
pub const META_FILLREGION: u32 = 552u32;
pub const META_FLOODFILL: u32 = 1049u32;
pub const META_FRAMEREGION: u32 = 1065u32;
pub const META_INTERSECTCLIPRECT: u32 = 1046u32;
pub const META_INVERTREGION: u32 = 298u32;
pub const META_LINETO: u32 = 531u32;
pub const META_MOVETO: u32 = 532u32;
pub const META_OFFSETCLIPRGN: u32 = 544u32;
pub const META_OFFSETVIEWPORTORG: u32 = 529u32;
pub const META_OFFSETWINDOWORG: u32 = 527u32;
pub const META_PAINTREGION: u32 = 299u32;
pub const META_PATBLT: u32 = 1565u32;
pub const META_PIE: u32 = 2074u32;
pub const META_POLYGON: u32 = 804u32;
pub const META_POLYLINE: u32 = 805u32;
pub const META_POLYPOLYGON: u32 = 1336u32;
pub const META_REALIZEPALETTE: u32 = 53u32;
pub const META_RECTANGLE: u32 = 1051u32;
pub const META_RESIZEPALETTE: u32 = 313u32;
pub const META_RESTOREDC: u32 = 295u32;
pub const META_ROUNDRECT: u32 = 1564u32;
pub const META_SAVEDC: u32 = 30u32;
pub const META_SCALEVIEWPORTEXT: u32 = 1042u32;
pub const META_SCALEWINDOWEXT: u32 = 1040u32;
pub const META_SELECTCLIPREGION: u32 = 300u32;
pub const META_SELECTOBJECT: u32 = 301u32;
pub const META_SELECTPALETTE: u32 = 564u32;
pub const META_SETBKCOLOR: u32 = 513u32;
pub const META_SETBKMODE: u32 = 258u32;
pub const META_SETDIBTODEV: u32 = 3379u32;
pub const META_SETLAYOUT: u32 = 329u32;
pub const META_SETMAPMODE: u32 = 259u32;
pub const META_SETMAPPERFLAGS: u32 = 561u32;
pub const META_SETPALENTRIES: u32 = 55u32;
pub const META_SETPIXEL: u32 = 1055u32;
pub const META_SETPOLYFILLMODE: u32 = 262u32;
pub const META_SETRELABS: u32 = 261u32;
pub const META_SETROP2: u32 = 260u32;
pub const META_SETSTRETCHBLTMODE: u32 = 263u32;
pub const META_SETTEXTALIGN: u32 = 302u32;
pub const META_SETTEXTCHAREXTRA: u32 = 264u32;
pub const META_SETTEXTCOLOR: u32 = 521u32;
pub const META_SETTEXTJUSTIFICATION: u32 = 522u32;
pub const META_SETVIEWPORTEXT: u32 = 526u32;
pub const META_SETVIEWPORTORG: u32 = 525u32;
pub const META_SETWINDOWEXT: u32 = 524u32;
pub const META_SETWINDOWORG: u32 = 523u32;
pub const META_STRETCHBLT: u32 = 2851u32;
pub const META_STRETCHDIB: u32 = 3907u32;
pub const META_TEXTOUT: u32 = 1313u32;
pub const MFCOMMENT: u32 = 15u32;
pub type MFENUMPROC = Option<unsafe extern "system" fn(hdc: HDC, lpht: *const HANDLETABLE, lpmr: *const METARECORD, nobj: i32, param4: super::super::Foundation::LPARAM) -> i32>;
pub const MILCORE_TS_QUERYVER_RESULT_FALSE: u32 = 0u32;
pub const MILCORE_TS_QUERYVER_RESULT_TRUE: u32 = 2147483647u32;
pub const MM_ANISOTROPIC: HDC_MAP_MODE = HDC_MAP_MODE(8i32);
pub const MM_HIENGLISH: HDC_MAP_MODE = HDC_MAP_MODE(5i32);
pub const MM_HIMETRIC: HDC_MAP_MODE = HDC_MAP_MODE(3i32);
pub const MM_ISOTROPIC: HDC_MAP_MODE = HDC_MAP_MODE(7i32);
pub const MM_LOENGLISH: HDC_MAP_MODE = HDC_MAP_MODE(4i32);
pub const MM_LOMETRIC: HDC_MAP_MODE = HDC_MAP_MODE(2i32);
pub const MM_MAX_AXES_NAMELEN: u32 = 16u32;
pub const MM_MAX_NUMAXES: u32 = 16u32;
pub const MM_TEXT: HDC_MAP_MODE = HDC_MAP_MODE(1i32);
pub const MM_TWIPS: HDC_MAP_MODE = HDC_MAP_MODE(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODIFY_WORLD_TRANSFORM_MODE(pub u32);
pub type MONITORENUMPROC = Option<unsafe extern "system" fn(param0: HMONITOR, param1: HDC, param2: *mut super::super::Foundation::RECT, param3: super::super::Foundation::LPARAM) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: super::super::Foundation::RECT,
    pub rcWork: super::super::Foundation::RECT,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONITORINFOEXA {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [i8; 32],
}
impl Default for MONITORINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MONITORINFOEXW {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [u16; 32],
}
impl Default for MONITORINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MONITOR_DEFAULTTONEAREST: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(2u32);
pub const MONITOR_DEFAULTTONULL: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(0u32);
pub const MONITOR_DEFAULTTOPRIMARY: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONITOR_FROM_FLAGS(pub u32);
pub const MONO_FONT: u32 = 8u32;
pub const MOUSETRAILS: u32 = 39u32;
pub const MWT_IDENTITY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(1u32);
pub const MWT_LEFTMULTIPLY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(2u32);
pub const MWT_RIGHTMULTIPLY: MODIFY_WORLD_TRANSFORM_MODE = MODIFY_WORLD_TRANSFORM_MODE(3u32);
pub const NEWFRAME: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NEWTEXTMETRICA {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: TMPF_FLAGS,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NEWTEXTMETRICW {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: TMPF_FLAGS,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
pub const NEWTRANSPARENT: u32 = 3u32;
pub const NEXTBAND: u32 = 3u32;
pub const NOMIRRORBITMAP: ROP_CODE = ROP_CODE(2147483648u32);
pub const NONANTIALIASED_QUALITY: FONT_QUALITY = FONT_QUALITY(3u8);
pub const NOTSRCCOPY: ROP_CODE = ROP_CODE(3342344u32);
pub const NOTSRCERASE: ROP_CODE = ROP_CODE(1114278u32);
pub const NTM_BOLD: i32 = 32i32;
pub const NTM_DSIG: u32 = 2097152u32;
pub const NTM_ITALIC: i32 = 1i32;
pub const NTM_MULTIPLEMASTER: u32 = 524288u32;
pub const NTM_NONNEGATIVE_AC: u32 = 65536u32;
pub const NTM_PS_OPENTYPE: u32 = 131072u32;
pub const NTM_REGULAR: i32 = 64i32;
pub const NTM_TT_OPENTYPE: u32 = 262144u32;
pub const NTM_TYPE1: u32 = 1048576u32;
pub const NULLREGION: GDI_REGION_TYPE = GDI_REGION_TYPE(1i32);
pub const NULL_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(5i32);
pub const NULL_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(8i32);
pub const NUMBRUSHES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(16u32);
pub const NUMCOLORS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(24u32);
pub const NUMFONTS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(22u32);
pub const NUMMARKERS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(20u32);
pub const NUMPENS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(18u32);
pub const NUMRESERVED: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(106u32);
pub const OBJ_BITMAP: OBJ_TYPE = OBJ_TYPE(7i32);
pub const OBJ_BRUSH: OBJ_TYPE = OBJ_TYPE(2i32);
pub const OBJ_COLORSPACE: OBJ_TYPE = OBJ_TYPE(14i32);
pub const OBJ_DC: OBJ_TYPE = OBJ_TYPE(3i32);
pub const OBJ_ENHMETADC: OBJ_TYPE = OBJ_TYPE(12i32);
pub const OBJ_ENHMETAFILE: OBJ_TYPE = OBJ_TYPE(13i32);
pub const OBJ_EXTPEN: OBJ_TYPE = OBJ_TYPE(11i32);
pub const OBJ_FONT: OBJ_TYPE = OBJ_TYPE(6i32);
pub const OBJ_MEMDC: OBJ_TYPE = OBJ_TYPE(10i32);
pub const OBJ_METADC: OBJ_TYPE = OBJ_TYPE(4i32);
pub const OBJ_METAFILE: OBJ_TYPE = OBJ_TYPE(9i32);
pub const OBJ_PAL: OBJ_TYPE = OBJ_TYPE(5i32);
pub const OBJ_PEN: OBJ_TYPE = OBJ_TYPE(1i32);
pub const OBJ_REGION: OBJ_TYPE = OBJ_TYPE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OBJ_TYPE(pub i32);
pub const OEM_CHARSET: FONT_CHARSET = FONT_CHARSET(255u8);
pub const OEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(10i32);
pub const OPAQUE: BACKGROUND_MODE = BACKGROUND_MODE(2u32);
pub const OPENCHANNEL: u32 = 4110u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OUTLINETEXTMETRICA {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICA,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: super::super::Foundation::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::super::Foundation::POINT,
    pub otmptSubscriptOffset: super::super::Foundation::POINT,
    pub otmptSuperscriptSize: super::super::Foundation::POINT,
    pub otmptSuperscriptOffset: super::super::Foundation::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: windows_core::PSTR,
    pub otmpFaceName: windows_core::PSTR,
    pub otmpStyleName: windows_core::PSTR,
    pub otmpFullName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OUTLINETEXTMETRICW {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICW,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: super::super::Foundation::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::super::Foundation::POINT,
    pub otmptSubscriptOffset: super::super::Foundation::POINT,
    pub otmptSuperscriptSize: super::super::Foundation::POINT,
    pub otmptSuperscriptOffset: super::super::Foundation::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: windows_core::PSTR,
    pub otmpFaceName: windows_core::PSTR,
    pub otmpStyleName: windows_core::PSTR,
    pub otmpFullName: windows_core::PSTR,
}
pub const OUT_CHARACTER_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(2u8);
pub const OUT_DEFAULT_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(0u8);
pub const OUT_DEVICE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(5u8);
pub const OUT_OUTLINE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(8u8);
pub const OUT_PS_ONLY_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(10u8);
pub const OUT_RASTER_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(6u8);
pub const OUT_SCREEN_OUTLINE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(9u8);
pub const OUT_STRING_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(1u8);
pub const OUT_STROKE_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(3u8);
pub const OUT_TT_ONLY_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(7u8);
pub const OUT_TT_PRECIS: FONT_OUTPUT_PRECISION = FONT_OUTPUT_PRECISION(4u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: windows_core::BOOL,
    pub rcPaint: super::super::Foundation::RECT,
    pub fRestore: windows_core::BOOL,
    pub fIncUpdate: windows_core::BOOL,
    pub rgbReserved: [u8; 32],
}
impl Default for PAINTSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PANOSE {
    pub bFamilyType: PAN_FAMILY_TYPE,
    pub bSerifStyle: PAN_SERIF_STYLE,
    pub bWeight: PAN_WEIGHT,
    pub bProportion: PAN_PROPORTION,
    pub bContrast: PAN_CONTRAST,
    pub bStrokeVariation: PAN_STROKE_VARIATION,
    pub bArmStyle: PAN_ARM_STYLE,
    pub bLetterform: PAN_LETT_FORM,
    pub bMidline: PAN_MIDLINE,
    pub bXHeight: PAN_XHEIGHT,
}
pub const PANOSE_COUNT: u32 = 10u32;
pub const PAN_ANY: u32 = 0u32;
pub const PAN_ARMSTYLE_INDEX: u32 = 6u32;
pub const PAN_ARM_ANY: PAN_ARM_STYLE = PAN_ARM_STYLE(0u8);
pub const PAN_ARM_NO_FIT: PAN_ARM_STYLE = PAN_ARM_STYLE(1u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_ARM_STYLE(pub u8);
pub const PAN_BENT_ARMS_DOUBLE_SERIF: PAN_ARM_STYLE = PAN_ARM_STYLE(11u8);
pub const PAN_BENT_ARMS_HORZ: PAN_ARM_STYLE = PAN_ARM_STYLE(7u8);
pub const PAN_BENT_ARMS_SINGLE_SERIF: PAN_ARM_STYLE = PAN_ARM_STYLE(10u8);
pub const PAN_BENT_ARMS_VERT: PAN_ARM_STYLE = PAN_ARM_STYLE(9u8);
pub const PAN_BENT_ARMS_WEDGE: PAN_ARM_STYLE = PAN_ARM_STYLE(8u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_CONTRAST(pub u8);
pub const PAN_CONTRAST_ANY: PAN_CONTRAST = PAN_CONTRAST(0u8);
pub const PAN_CONTRAST_HIGH: PAN_CONTRAST = PAN_CONTRAST(8u8);
pub const PAN_CONTRAST_INDEX: PAN_CONTRAST = PAN_CONTRAST(4u8);
pub const PAN_CONTRAST_LOW: PAN_CONTRAST = PAN_CONTRAST(4u8);
pub const PAN_CONTRAST_MEDIUM: PAN_CONTRAST = PAN_CONTRAST(6u8);
pub const PAN_CONTRAST_MEDIUM_HIGH: PAN_CONTRAST = PAN_CONTRAST(7u8);
pub const PAN_CONTRAST_MEDIUM_LOW: PAN_CONTRAST = PAN_CONTRAST(5u8);
pub const PAN_CONTRAST_NONE: PAN_CONTRAST = PAN_CONTRAST(2u8);
pub const PAN_CONTRAST_NO_FIT: PAN_CONTRAST = PAN_CONTRAST(1u8);
pub const PAN_CONTRAST_VERY_HIGH: PAN_CONTRAST = PAN_CONTRAST(9u8);
pub const PAN_CONTRAST_VERY_LOW: PAN_CONTRAST = PAN_CONTRAST(3u8);
pub const PAN_CULTURE_LATIN: u32 = 0u32;
pub const PAN_FAMILYTYPE_INDEX: u32 = 0u32;
pub const PAN_FAMILY_ANY: PAN_FAMILY_TYPE = PAN_FAMILY_TYPE(0u8);
pub const PAN_FAMILY_DECORATIVE: PAN_FAMILY_TYPE = PAN_FAMILY_TYPE(4u8);
pub const PAN_FAMILY_NO_FIT: PAN_FAMILY_TYPE = PAN_FAMILY_TYPE(1u8);
pub const PAN_FAMILY_PICTORIAL: PAN_FAMILY_TYPE = PAN_FAMILY_TYPE(5u8);
pub const PAN_FAMILY_SCRIPT: PAN_FAMILY_TYPE = PAN_FAMILY_TYPE(3u8);
pub const PAN_FAMILY_TEXT_DISPLAY: PAN_FAMILY_TYPE = PAN_FAMILY_TYPE(2u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_FAMILY_TYPE(pub u8);
pub const PAN_LETTERFORM_INDEX: u32 = 7u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_LETT_FORM(pub u8);
pub const PAN_LETT_FORM_ANY: PAN_LETT_FORM = PAN_LETT_FORM(0u8);
pub const PAN_LETT_FORM_NO_FIT: PAN_LETT_FORM = PAN_LETT_FORM(1u8);
pub const PAN_LETT_NORMAL_BOXED: PAN_LETT_FORM = PAN_LETT_FORM(4u8);
pub const PAN_LETT_NORMAL_CONTACT: PAN_LETT_FORM = PAN_LETT_FORM(2u8);
pub const PAN_LETT_NORMAL_FLATTENED: PAN_LETT_FORM = PAN_LETT_FORM(5u8);
pub const PAN_LETT_NORMAL_OFF_CENTER: PAN_LETT_FORM = PAN_LETT_FORM(7u8);
pub const PAN_LETT_NORMAL_ROUNDED: PAN_LETT_FORM = PAN_LETT_FORM(6u8);
pub const PAN_LETT_NORMAL_SQUARE: PAN_LETT_FORM = PAN_LETT_FORM(8u8);
pub const PAN_LETT_NORMAL_WEIGHTED: PAN_LETT_FORM = PAN_LETT_FORM(3u8);
pub const PAN_LETT_OBLIQUE_BOXED: PAN_LETT_FORM = PAN_LETT_FORM(11u8);
pub const PAN_LETT_OBLIQUE_CONTACT: PAN_LETT_FORM = PAN_LETT_FORM(9u8);
pub const PAN_LETT_OBLIQUE_FLATTENED: PAN_LETT_FORM = PAN_LETT_FORM(12u8);
pub const PAN_LETT_OBLIQUE_OFF_CENTER: PAN_LETT_FORM = PAN_LETT_FORM(14u8);
pub const PAN_LETT_OBLIQUE_ROUNDED: PAN_LETT_FORM = PAN_LETT_FORM(13u8);
pub const PAN_LETT_OBLIQUE_SQUARE: PAN_LETT_FORM = PAN_LETT_FORM(15u8);
pub const PAN_LETT_OBLIQUE_WEIGHTED: PAN_LETT_FORM = PAN_LETT_FORM(10u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_MIDLINE(pub u8);
pub const PAN_MIDLINE_ANY: PAN_MIDLINE = PAN_MIDLINE(0u8);
pub const PAN_MIDLINE_CONSTANT_POINTED: PAN_MIDLINE = PAN_MIDLINE(9u8);
pub const PAN_MIDLINE_CONSTANT_SERIFED: PAN_MIDLINE = PAN_MIDLINE(10u8);
pub const PAN_MIDLINE_CONSTANT_TRIMMED: PAN_MIDLINE = PAN_MIDLINE(8u8);
pub const PAN_MIDLINE_HIGH_POINTED: PAN_MIDLINE = PAN_MIDLINE(6u8);
pub const PAN_MIDLINE_HIGH_SERIFED: PAN_MIDLINE = PAN_MIDLINE(7u8);
pub const PAN_MIDLINE_HIGH_TRIMMED: PAN_MIDLINE = PAN_MIDLINE(5u8);
pub const PAN_MIDLINE_INDEX: PAN_MIDLINE = PAN_MIDLINE(8u8);
pub const PAN_MIDLINE_LOW_POINTED: PAN_MIDLINE = PAN_MIDLINE(12u8);
pub const PAN_MIDLINE_LOW_SERIFED: PAN_MIDLINE = PAN_MIDLINE(13u8);
pub const PAN_MIDLINE_LOW_TRIMMED: PAN_MIDLINE = PAN_MIDLINE(11u8);
pub const PAN_MIDLINE_NO_FIT: PAN_MIDLINE = PAN_MIDLINE(1u8);
pub const PAN_MIDLINE_STANDARD_POINTED: PAN_MIDLINE = PAN_MIDLINE(3u8);
pub const PAN_MIDLINE_STANDARD_SERIFED: PAN_MIDLINE = PAN_MIDLINE(4u8);
pub const PAN_MIDLINE_STANDARD_TRIMMED: PAN_MIDLINE = PAN_MIDLINE(2u8);
pub const PAN_NO_FIT: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_PROPORTION(pub u8);
pub const PAN_PROPORTION_INDEX: u32 = 3u32;
pub const PAN_PROP_ANY: PAN_PROPORTION = PAN_PROPORTION(0u8);
pub const PAN_PROP_CONDENSED: PAN_PROPORTION = PAN_PROPORTION(6u8);
pub const PAN_PROP_EVEN_WIDTH: PAN_PROPORTION = PAN_PROPORTION(4u8);
pub const PAN_PROP_EXPANDED: PAN_PROPORTION = PAN_PROPORTION(5u8);
pub const PAN_PROP_MODERN: PAN_PROPORTION = PAN_PROPORTION(3u8);
pub const PAN_PROP_MONOSPACED: PAN_PROPORTION = PAN_PROPORTION(9u8);
pub const PAN_PROP_NO_FIT: PAN_PROPORTION = PAN_PROPORTION(1u8);
pub const PAN_PROP_OLD_STYLE: PAN_PROPORTION = PAN_PROPORTION(2u8);
pub const PAN_PROP_VERY_CONDENSED: PAN_PROPORTION = PAN_PROPORTION(8u8);
pub const PAN_PROP_VERY_EXPANDED: PAN_PROPORTION = PAN_PROPORTION(7u8);
pub const PAN_SERIFSTYLE_INDEX: u32 = 1u32;
pub const PAN_SERIF_ANY: PAN_SERIF_STYLE = PAN_SERIF_STYLE(0u8);
pub const PAN_SERIF_BONE: PAN_SERIF_STYLE = PAN_SERIF_STYLE(8u8);
pub const PAN_SERIF_COVE: PAN_SERIF_STYLE = PAN_SERIF_STYLE(2u8);
pub const PAN_SERIF_EXAGGERATED: PAN_SERIF_STYLE = PAN_SERIF_STYLE(9u8);
pub const PAN_SERIF_FLARED: PAN_SERIF_STYLE = PAN_SERIF_STYLE(14u8);
pub const PAN_SERIF_NORMAL_SANS: PAN_SERIF_STYLE = PAN_SERIF_STYLE(11u8);
pub const PAN_SERIF_NO_FIT: PAN_SERIF_STYLE = PAN_SERIF_STYLE(1u8);
pub const PAN_SERIF_OBTUSE_COVE: PAN_SERIF_STYLE = PAN_SERIF_STYLE(3u8);
pub const PAN_SERIF_OBTUSE_SANS: PAN_SERIF_STYLE = PAN_SERIF_STYLE(12u8);
pub const PAN_SERIF_OBTUSE_SQUARE_COVE: PAN_SERIF_STYLE = PAN_SERIF_STYLE(5u8);
pub const PAN_SERIF_PERP_SANS: PAN_SERIF_STYLE = PAN_SERIF_STYLE(13u8);
pub const PAN_SERIF_ROUNDED: PAN_SERIF_STYLE = PAN_SERIF_STYLE(15u8);
pub const PAN_SERIF_SQUARE: PAN_SERIF_STYLE = PAN_SERIF_STYLE(6u8);
pub const PAN_SERIF_SQUARE_COVE: PAN_SERIF_STYLE = PAN_SERIF_STYLE(4u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_SERIF_STYLE(pub u8);
pub const PAN_SERIF_THIN: PAN_SERIF_STYLE = PAN_SERIF_STYLE(7u8);
pub const PAN_SERIF_TRIANGLE: PAN_SERIF_STYLE = PAN_SERIF_STYLE(10u8);
pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: PAN_ARM_STYLE = PAN_ARM_STYLE(6u8);
pub const PAN_STRAIGHT_ARMS_HORZ: PAN_ARM_STYLE = PAN_ARM_STYLE(2u8);
pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: PAN_ARM_STYLE = PAN_ARM_STYLE(5u8);
pub const PAN_STRAIGHT_ARMS_VERT: PAN_ARM_STYLE = PAN_ARM_STYLE(4u8);
pub const PAN_STRAIGHT_ARMS_WEDGE: PAN_ARM_STYLE = PAN_ARM_STYLE(3u8);
pub const PAN_STROKEVARIATION_INDEX: u32 = 5u32;
pub const PAN_STROKE_ANY: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(0u8);
pub const PAN_STROKE_GRADUAL_DIAG: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(2u8);
pub const PAN_STROKE_GRADUAL_HORZ: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(5u8);
pub const PAN_STROKE_GRADUAL_TRAN: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(3u8);
pub const PAN_STROKE_GRADUAL_VERT: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(4u8);
pub const PAN_STROKE_INSTANT_VERT: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(8u8);
pub const PAN_STROKE_NO_FIT: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(1u8);
pub const PAN_STROKE_RAPID_HORZ: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(7u8);
pub const PAN_STROKE_RAPID_VERT: PAN_STROKE_VARIATION = PAN_STROKE_VARIATION(6u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_STROKE_VARIATION(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_WEIGHT(pub u8);
pub const PAN_WEIGHT_ANY: PAN_WEIGHT = PAN_WEIGHT(0u8);
pub const PAN_WEIGHT_BLACK: PAN_WEIGHT = PAN_WEIGHT(10u8);
pub const PAN_WEIGHT_BOLD: PAN_WEIGHT = PAN_WEIGHT(8u8);
pub const PAN_WEIGHT_BOOK: PAN_WEIGHT = PAN_WEIGHT(5u8);
pub const PAN_WEIGHT_DEMI: PAN_WEIGHT = PAN_WEIGHT(7u8);
pub const PAN_WEIGHT_HEAVY: PAN_WEIGHT = PAN_WEIGHT(9u8);
pub const PAN_WEIGHT_INDEX: PAN_WEIGHT = PAN_WEIGHT(2u8);
pub const PAN_WEIGHT_LIGHT: PAN_WEIGHT = PAN_WEIGHT(3u8);
pub const PAN_WEIGHT_MEDIUM: PAN_WEIGHT = PAN_WEIGHT(6u8);
pub const PAN_WEIGHT_NORD: PAN_WEIGHT = PAN_WEIGHT(11u8);
pub const PAN_WEIGHT_NO_FIT: PAN_WEIGHT = PAN_WEIGHT(1u8);
pub const PAN_WEIGHT_THIN: PAN_WEIGHT = PAN_WEIGHT(4u8);
pub const PAN_WEIGHT_VERY_LIGHT: PAN_WEIGHT = PAN_WEIGHT(2u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAN_XHEIGHT(pub u8);
pub const PAN_XHEIGHT_ANY: PAN_XHEIGHT = PAN_XHEIGHT(0u8);
pub const PAN_XHEIGHT_CONSTANT_LARGE: PAN_XHEIGHT = PAN_XHEIGHT(4u8);
pub const PAN_XHEIGHT_CONSTANT_SMALL: PAN_XHEIGHT = PAN_XHEIGHT(2u8);
pub const PAN_XHEIGHT_CONSTANT_STD: PAN_XHEIGHT = PAN_XHEIGHT(3u8);
pub const PAN_XHEIGHT_DUCKING_LARGE: PAN_XHEIGHT = PAN_XHEIGHT(7u8);
pub const PAN_XHEIGHT_DUCKING_SMALL: PAN_XHEIGHT = PAN_XHEIGHT(5u8);
pub const PAN_XHEIGHT_DUCKING_STD: PAN_XHEIGHT = PAN_XHEIGHT(6u8);
pub const PAN_XHEIGHT_INDEX: PAN_XHEIGHT = PAN_XHEIGHT(9u8);
pub const PAN_XHEIGHT_NO_FIT: PAN_XHEIGHT = PAN_XHEIGHT(1u8);
pub const PASSTHROUGH: u32 = 19u32;
pub const PATCOPY: ROP_CODE = ROP_CODE(15728673u32);
pub const PATINVERT: ROP_CODE = ROP_CODE(5898313u32);
pub const PATPAINT: ROP_CODE = ROP_CODE(16452105u32);
pub const PC_EXPLICIT: u32 = 2u32;
pub const PC_INTERIORS: u32 = 128u32;
pub const PC_NOCOLLAPSE: u32 = 4u32;
pub const PC_NONE: u32 = 0u32;
pub const PC_PATHS: u32 = 512u32;
pub const PC_POLYGON: u32 = 1u32;
pub const PC_POLYPOLYGON: u32 = 256u32;
pub const PC_RECTANGLE: u32 = 2u32;
pub const PC_RESERVED: u32 = 1u32;
pub const PC_SCANLINE: u32 = 8u32;
pub const PC_STYLED: u32 = 32u32;
pub const PC_TRAPEZOID: u32 = 4u32;
pub const PC_WIDE: u32 = 16u32;
pub const PC_WIDESTYLED: u32 = 64u32;
pub const PC_WINDPOLYGON: u32 = 4u32;
pub const PDEVICESIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(26u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PELARRAY {
    pub paXCount: i32,
    pub paYCount: i32,
    pub paXExt: i32,
    pub paYExt: i32,
    pub paRGBs: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PEN_STYLE(pub i32);
impl PEN_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PEN_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PEN_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PEN_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PEN_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PEN_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PHYSICALHEIGHT: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(111u32);
pub const PHYSICALOFFSETX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(112u32);
pub const PHYSICALOFFSETY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(113u32);
pub const PHYSICALWIDTH: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(110u32);
pub const PLANES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(14u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTFX {
    pub x: FIXED,
    pub y: FIXED,
}
pub const POLYFILL_LAST: u32 = 2u32;
pub const POLYGONALCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(32u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POLYTEXTA {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: windows_core::PCSTR,
    pub uiFlags: u32,
    pub rcl: super::super::Foundation::RECT,
    pub pdx: *mut i32,
}
impl Default for POLYTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POLYTEXTW {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: windows_core::PCWSTR,
    pub uiFlags: u32,
    pub rcl: super::super::Foundation::RECT,
    pub pdx: *mut i32,
}
impl Default for POLYTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POSTSCRIPT_DATA: u32 = 37u32;
pub const POSTSCRIPT_IDENTIFY: u32 = 4117u32;
pub const POSTSCRIPT_IGNORE: u32 = 38u32;
pub const POSTSCRIPT_INJECTION: u32 = 4118u32;
pub const POSTSCRIPT_PASSTHROUGH: u32 = 4115u32;
pub const PRINTRATEUNIT_CPS: u32 = 2u32;
pub const PRINTRATEUNIT_IPM: u32 = 4u32;
pub const PRINTRATEUNIT_LPM: u32 = 3u32;
pub const PRINTRATEUNIT_PPM: u32 = 1u32;
pub const PROOF_QUALITY: FONT_QUALITY = FONT_QUALITY(2u8);
pub const PR_JOBSTATUS: u32 = 0u32;
pub const PSIDENT_GDICENTRIC: u32 = 0u32;
pub const PSIDENT_PSCENTRIC: u32 = 1u32;
pub const PSINJECT_DLFONT: u32 = 3722304989u32;
pub const PSPROTOCOL_ASCII: u32 = 0u32;
pub const PSPROTOCOL_BCP: u32 = 1u32;
pub const PSPROTOCOL_BINARY: u32 = 3u32;
pub const PSPROTOCOL_TBCP: u32 = 2u32;
pub const PS_ALTERNATE: PEN_STYLE = PEN_STYLE(8i32);
pub const PS_COSMETIC: PEN_STYLE = PEN_STYLE(0i32);
pub const PS_DASH: PEN_STYLE = PEN_STYLE(1i32);
pub const PS_DASHDOT: PEN_STYLE = PEN_STYLE(3i32);
pub const PS_DASHDOTDOT: PEN_STYLE = PEN_STYLE(4i32);
pub const PS_DOT: PEN_STYLE = PEN_STYLE(2i32);
pub const PS_ENDCAP_FLAT: PEN_STYLE = PEN_STYLE(512i32);
pub const PS_ENDCAP_MASK: PEN_STYLE = PEN_STYLE(3840i32);
pub const PS_ENDCAP_ROUND: PEN_STYLE = PEN_STYLE(0i32);
pub const PS_ENDCAP_SQUARE: PEN_STYLE = PEN_STYLE(256i32);
pub const PS_GEOMETRIC: PEN_STYLE = PEN_STYLE(65536i32);
pub const PS_INSIDEFRAME: PEN_STYLE = PEN_STYLE(6i32);
pub const PS_JOIN_BEVEL: PEN_STYLE = PEN_STYLE(4096i32);
pub const PS_JOIN_MASK: PEN_STYLE = PEN_STYLE(61440i32);
pub const PS_JOIN_MITER: PEN_STYLE = PEN_STYLE(8192i32);
pub const PS_JOIN_ROUND: PEN_STYLE = PEN_STYLE(0i32);
pub const PS_NULL: PEN_STYLE = PEN_STYLE(5i32);
pub const PS_SOLID: PEN_STYLE = PEN_STYLE(0i32);
pub const PS_STYLE_MASK: PEN_STYLE = PEN_STYLE(15i32);
pub const PS_TYPE_MASK: PEN_STYLE = PEN_STYLE(983040i32);
pub const PS_USERSTYLE: PEN_STYLE = PEN_STYLE(7i32);
pub const PT_BEZIERTO: u32 = 4u32;
pub const PT_CLOSEFIGURE: u32 = 1u32;
pub const PT_LINETO: u32 = 2u32;
pub const PT_MOVETO: u32 = 6u32;
pub const QDI_DIBTOSCREEN: u32 = 4u32;
pub const QDI_GETDIBITS: u32 = 2u32;
pub const QDI_SETDIBITS: u32 = 1u32;
pub const QDI_STRETCHDIB: u32 = 8u32;
pub const QUERYDIBSUPPORT: u32 = 3073u32;
pub const QUERYESCSUPPORT: u32 = 8u32;
pub const QUERYROPSUPPORT: u32 = 40u32;
pub const R2_BLACK: R2_MODE = R2_MODE(1i32);
pub const R2_COPYPEN: R2_MODE = R2_MODE(13i32);
pub const R2_LAST: R2_MODE = R2_MODE(16i32);
pub const R2_MASKNOTPEN: R2_MODE = R2_MODE(3i32);
pub const R2_MASKPEN: R2_MODE = R2_MODE(9i32);
pub const R2_MASKPENNOT: R2_MODE = R2_MODE(5i32);
pub const R2_MERGENOTPEN: R2_MODE = R2_MODE(12i32);
pub const R2_MERGEPEN: R2_MODE = R2_MODE(15i32);
pub const R2_MERGEPENNOT: R2_MODE = R2_MODE(14i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct R2_MODE(pub i32);
pub const R2_NOP: R2_MODE = R2_MODE(11i32);
pub const R2_NOT: R2_MODE = R2_MODE(6i32);
pub const R2_NOTCOPYPEN: R2_MODE = R2_MODE(4i32);
pub const R2_NOTMASKPEN: R2_MODE = R2_MODE(8i32);
pub const R2_NOTMERGEPEN: R2_MODE = R2_MODE(2i32);
pub const R2_NOTXORPEN: R2_MODE = R2_MODE(10i32);
pub const R2_WHITE: R2_MODE = R2_MODE(16i32);
pub const R2_XORPEN: R2_MODE = R2_MODE(7i32);
pub const RASTERCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(38u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RASTERIZER_STATUS {
    pub nSize: i16,
    pub wFlags: i16,
    pub nLanguageID: i16,
}
pub const RASTER_FONTTYPE: u32 = 1u32;
pub const RC_BANDING: u32 = 2u32;
pub const RC_BIGFONT: u32 = 1024u32;
pub const RC_BITBLT: u32 = 1u32;
pub const RC_BITMAP64: u32 = 8u32;
pub const RC_DEVBITS: u32 = 32768u32;
pub const RC_DIBTODEV: u32 = 512u32;
pub const RC_DI_BITMAP: u32 = 128u32;
pub const RC_FLOODFILL: u32 = 4096u32;
pub const RC_GDI20_OUTPUT: u32 = 16u32;
pub const RC_GDI20_STATE: u32 = 32u32;
pub const RC_OP_DX_OUTPUT: u32 = 16384u32;
pub const RC_PALETTE: u32 = 256u32;
pub const RC_SAVEBITMAP: u32 = 64u32;
pub const RC_SCALING: u32 = 4u32;
pub const RC_STRETCHBLT: u32 = 2048u32;
pub const RC_STRETCHDIB: u32 = 8192u32;
pub const RDH_RECTANGLES: u32 = 1u32;
pub const RDW_ALLCHILDREN: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(128u32);
pub const RDW_ERASE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(4u32);
pub const RDW_ERASENOW: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(512u32);
pub const RDW_FRAME: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(1024u32);
pub const RDW_INTERNALPAINT: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(2u32);
pub const RDW_INVALIDATE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(1u32);
pub const RDW_NOCHILDREN: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(64u32);
pub const RDW_NOERASE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(32u32);
pub const RDW_NOFRAME: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(2048u32);
pub const RDW_NOINTERNALPAINT: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(16u32);
pub const RDW_UPDATENOW: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(256u32);
pub const RDW_VALIDATE: REDRAW_WINDOW_FLAGS = REDRAW_WINDOW_FLAGS(8u32);
pub type READEMBEDPROC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void, param2: u32) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REDRAW_WINDOW_FLAGS(pub u32);
impl REDRAW_WINDOW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REDRAW_WINDOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REDRAW_WINDOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RELATIVE: u32 = 2u32;
pub const RESTORE_CTM: u32 = 4100u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RGBTRIPLE {
    pub rgbtBlue: u8,
    pub rgbtGreen: u8,
    pub rgbtRed: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGNDATA {
    pub rdh: RGNDATAHEADER,
    pub Buffer: [i8; 1],
}
impl Default for RGNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RGNDATAHEADER {
    pub dwSize: u32,
    pub iType: u32,
    pub nCount: u32,
    pub nRgnSize: u32,
    pub rcBound: super::super::Foundation::RECT,
}
pub const RGN_AND: RGN_COMBINE_MODE = RGN_COMBINE_MODE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RGN_COMBINE_MODE(pub i32);
pub const RGN_COPY: RGN_COMBINE_MODE = RGN_COMBINE_MODE(5i32);
pub const RGN_DIFF: RGN_COMBINE_MODE = RGN_COMBINE_MODE(4i32);
pub const RGN_ERROR: GDI_REGION_TYPE = GDI_REGION_TYPE(0i32);
pub const RGN_MAX: RGN_COMBINE_MODE = RGN_COMBINE_MODE(5i32);
pub const RGN_MIN: RGN_COMBINE_MODE = RGN_COMBINE_MODE(1i32);
pub const RGN_OR: RGN_COMBINE_MODE = RGN_COMBINE_MODE(2i32);
pub const RGN_XOR: RGN_COMBINE_MODE = RGN_COMBINE_MODE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROP_CODE(pub u32);
impl ROP_CODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ROP_CODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ROP_CODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ROP_CODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ROP_CODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ROP_CODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RUSSIAN_CHARSET: FONT_CHARSET = FONT_CHARSET(204u8);
pub const SAVE_CTM: u32 = 4101u32;
pub const SB_CONST_ALPHA: u32 = 1u32;
pub const SB_GRAD_RECT: u32 = 16u32;
pub const SB_GRAD_TRI: u32 = 32u32;
pub const SB_NONE: u32 = 0u32;
pub const SB_PIXEL_ALPHA: u32 = 2u32;
pub const SB_PREMULT_ALPHA: u32 = 4u32;
pub const SCALINGFACTORX: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(114u32);
pub const SCALINGFACTORY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(115u32);
pub const SC_SCREENSAVE: u32 = 61760u32;
pub const SELECTDIB: u32 = 41u32;
pub const SELECTPAPERSOURCE: u32 = 18u32;
pub const SETABORTPROC: u32 = 9u32;
pub const SETALLJUSTVALUES: u32 = 771u32;
pub const SETCHARSET: u32 = 772u32;
pub const SETCOLORTABLE: u32 = 4u32;
pub const SETCOPYCOUNT: u32 = 17u32;
pub const SETDIBSCALING: u32 = 32u32;
pub const SETICMPROFILE_EMBEDED: u32 = 1u32;
pub const SETKERNTRACK: u32 = 770u32;
pub const SETLINECAP: u32 = 21u32;
pub const SETLINEJOIN: u32 = 22u32;
pub const SETMITERLIMIT: u32 = 23u32;
pub const SET_ARC_DIRECTION: u32 = 4102u32;
pub const SET_BACKGROUND_COLOR: u32 = 4103u32;
pub const SET_BOUNDS: u32 = 4109u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_BOUNDS_RECT_FLAGS(pub u32);
pub const SET_CLIP_BOX: u32 = 4108u32;
pub const SET_MIRROR_MODE: u32 = 4110u32;
pub const SET_POLY_MODE: u32 = 4104u32;
pub const SET_SCREEN_ANGLE: u32 = 4105u32;
pub const SET_SPREAD: u32 = 4106u32;
pub const SHADEBLENDCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(120u32);
pub const SHIFTJIS_CHARSET: FONT_CHARSET = FONT_CHARSET(128u8);
pub const SIMPLEREGION: GDI_REGION_TYPE = GDI_REGION_TYPE(2i32);
pub const SIZEPALETTE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(104u32);
pub const SPCLPASSTHROUGH2: u32 = 4568u32;
pub const SP_APPABORT: i32 = -2i32;
pub const SP_ERROR: i32 = -1i32;
pub const SP_NOTREPORTED: u32 = 16384u32;
pub const SP_OUTOFDISK: i32 = -4i32;
pub const SP_OUTOFMEMORY: i32 = -5i32;
pub const SP_USERABORT: i32 = -3i32;
pub const SRCAND: ROP_CODE = ROP_CODE(8913094u32);
pub const SRCCOPY: ROP_CODE = ROP_CODE(13369376u32);
pub const SRCERASE: ROP_CODE = ROP_CODE(4457256u32);
pub const SRCINVERT: ROP_CODE = ROP_CODE(6684742u32);
pub const SRCPAINT: ROP_CODE = ROP_CODE(15597702u32);
pub const STARTDOC: u32 = 10u32;
pub const STOCK_LAST: u32 = 19u32;
pub const STRETCHBLT: u32 = 2048u32;
pub const STRETCH_ANDSCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STRETCH_BLT_MODE(pub i32);
pub const STRETCH_DELETESCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(3i32);
pub const STRETCH_HALFTONE: STRETCH_BLT_MODE = STRETCH_BLT_MODE(4i32);
pub const STRETCH_ORSCANS: STRETCH_BLT_MODE = STRETCH_BLT_MODE(2i32);
pub const SYMBOL_CHARSET: FONT_CHARSET = FONT_CHARSET(2u8);
pub const SYSPAL_ERROR: u32 = 0u32;
pub const SYSPAL_NOSTATIC: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(2u32);
pub const SYSPAL_NOSTATIC256: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(3u32);
pub const SYSPAL_STATIC: SYSTEM_PALETTE_USE = SYSTEM_PALETTE_USE(1u32);
pub const SYSRGN: u32 = 4u32;
pub const SYSTEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(16i32);
pub const SYSTEM_FONT: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(13i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_PALETTE_USE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYS_COLOR_INDEX(pub i32);
pub const TA_BASELINE: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(24u32);
pub const TA_BOTTOM: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(8u32);
pub const TA_CENTER: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(6u32);
pub const TA_LEFT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
pub const TA_MASK: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(287u32);
pub const TA_NOUPDATECP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
pub const TA_RIGHT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(2u32);
pub const TA_RTLREADING: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(256u32);
pub const TA_TOP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
pub const TA_UPDATECP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(1u32);
pub const TC_CP_STROKE: u32 = 4u32;
pub const TC_CR_90: u32 = 8u32;
pub const TC_CR_ANY: u32 = 16u32;
pub const TC_EA_DOUBLE: u32 = 512u32;
pub const TC_IA_ABLE: u32 = 1024u32;
pub const TC_OP_CHARACTER: u32 = 1u32;
pub const TC_OP_STROKE: u32 = 2u32;
pub const TC_RA_ABLE: u32 = 8192u32;
pub const TC_RESERVED: u32 = 32768u32;
pub const TC_SA_CONTIN: u32 = 256u32;
pub const TC_SA_DOUBLE: u32 = 64u32;
pub const TC_SA_INTEGER: u32 = 128u32;
pub const TC_SCROLLBLT: u32 = 65536u32;
pub const TC_SF_X_YINDEP: u32 = 32u32;
pub const TC_SO_ABLE: u32 = 4096u32;
pub const TC_UA_ABLE: u32 = 2048u32;
pub const TC_VA_ABLE: u32 = 16384u32;
pub const TECHNOLOGY: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(2u32);
pub const TEXTCAPS: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(34u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TEXTMETRICA {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: TMPF_FLAGS,
    pub tmCharSet: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TEXTMETRICW {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: TMPF_FLAGS,
    pub tmCharSet: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEXT_ALIGN_OPTIONS(pub u32);
impl TEXT_ALIGN_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TEXT_ALIGN_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TEXT_ALIGN_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const THAI_CHARSET: FONT_CHARSET = FONT_CHARSET(222u8);
pub const TMPF_DEVICE: TMPF_FLAGS = TMPF_FLAGS(8u8);
pub const TMPF_FIXED_PITCH: TMPF_FLAGS = TMPF_FLAGS(1u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TMPF_FLAGS(pub u8);
impl TMPF_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TMPF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TMPF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TMPF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TMPF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TMPF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TMPF_TRUETYPE: TMPF_FLAGS = TMPF_FLAGS(4u8);
pub const TMPF_VECTOR: TMPF_FLAGS = TMPF_FLAGS(2u8);
pub const TRANSFORM_CTM: u32 = 4107u32;
pub const TRANSPARENT: BACKGROUND_MODE = BACKGROUND_MODE(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRIVERTEX {
    pub x: i32,
    pub y: i32,
    pub Red: u16,
    pub Green: u16,
    pub Blue: u16,
    pub Alpha: u16,
}
pub const TRUETYPE_FONTTYPE: u32 = 4u32;
pub const TTDELETE_DONTREMOVEFONT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTEMBEDINFO {
    pub usStructSize: u16,
    pub usRootStrSize: u16,
    pub pusRootStr: *mut u16,
}
impl Default for TTEMBEDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TTEMBED_EMBEDEUDC: TTEMBED_FLAGS = TTEMBED_FLAGS(32u32);
pub const TTEMBED_EUDCEMBEDDED: u32 = 2u32;
pub const TTEMBED_FAILIFVARIATIONSIMULATED: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TTEMBED_FLAGS(pub u32);
impl TTEMBED_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TTEMBED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TTEMBED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TTEMBED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TTEMBED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TTEMBED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TTEMBED_RAW: TTEMBED_FLAGS = TTEMBED_FLAGS(0u32);
pub const TTEMBED_SUBSET: TTEMBED_FLAGS = TTEMBED_FLAGS(1u32);
pub const TTEMBED_SUBSETCANCEL: u32 = 4u32;
pub const TTEMBED_TTCOMPRESSED: TTEMBED_FLAGS = TTEMBED_FLAGS(4u32);
pub const TTEMBED_VARIATIONSIMULATED: u32 = 1u32;
pub const TTEMBED_WEBOBJECT: u32 = 128u32;
pub const TTEMBED_XORENCRYPTDATA: u32 = 268435456u32;
pub const TTFCFP_APPLE_PLATFORMID: u32 = 1u32;
pub const TTFCFP_DELTA: u32 = 2u32;
pub const TTFCFP_DONT_CARE: u32 = 65535u32;
pub const TTFCFP_FLAGS_COMPRESS: u32 = 2u32;
pub const TTFCFP_FLAGS_GLYPHLIST: u32 = 8u32;
pub const TTFCFP_FLAGS_SUBSET: u32 = 1u32;
pub const TTFCFP_FLAGS_TTC: u32 = 4u32;
pub const TTFCFP_ISO_PLATFORMID: CREATE_FONT_PACKAGE_SUBSET_PLATFORM = CREATE_FONT_PACKAGE_SUBSET_PLATFORM(2i16);
pub const TTFCFP_LANG_KEEP_ALL: u32 = 0u32;
pub const TTFCFP_MS_PLATFORMID: u32 = 3u32;
pub const TTFCFP_STD_MAC_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = CREATE_FONT_PACKAGE_SUBSET_ENCODING(0i16);
pub const TTFCFP_SUBSET: u32 = 0u32;
pub const TTFCFP_SUBSET1: u32 = 1u32;
pub const TTFCFP_SYMBOL_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = CREATE_FONT_PACKAGE_SUBSET_ENCODING(0i16);
pub const TTFCFP_UNICODE_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = CREATE_FONT_PACKAGE_SUBSET_ENCODING(1i16);
pub const TTFCFP_UNICODE_PLATFORMID: CREATE_FONT_PACKAGE_SUBSET_PLATFORM = CREATE_FONT_PACKAGE_SUBSET_PLATFORM(0i16);
pub const TTFMFP_DELTA: u32 = 2u32;
pub const TTFMFP_SUBSET: u32 = 0u32;
pub const TTFMFP_SUBSET1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTLOADINFO {
    pub usStructSize: u16,
    pub usRefStrSize: u16,
    pub pusRefStr: *mut u16,
}
impl Default for TTLOADINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TTLOAD_EMBEDDED_FONT_STATUS(pub u32);
impl TTLOAD_EMBEDDED_FONT_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TTLOAD_EMBEDDED_FONT_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TTLOAD_EMBEDDED_FONT_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TTLOAD_EUDC_OVERWRITE: u32 = 2u32;
pub const TTLOAD_EUDC_SET: u32 = 4u32;
pub const TTLOAD_FONT_IN_SYSSTARTUP: TTLOAD_EMBEDDED_FONT_STATUS = TTLOAD_EMBEDDED_FONT_STATUS(2u32);
pub const TTLOAD_FONT_SUBSETTED: TTLOAD_EMBEDDED_FONT_STATUS = TTLOAD_EMBEDDED_FONT_STATUS(1u32);
pub const TTLOAD_PRIVATE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTPOLYCURVE {
    pub wType: u16,
    pub cpfx: u16,
    pub apfx: [POINTFX; 1],
}
impl Default for TTPOLYCURVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TTPOLYGONHEADER {
    pub cb: u32,
    pub dwType: u32,
    pub pfxStart: POINTFX,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTVALIDATIONTESTSPARAMS {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pusCharCodeSet: *mut u16,
}
impl Default for TTVALIDATIONTESTSPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TTVALIDATIONTESTSPARAMSEX {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pulCharCodeSet: *mut u32,
}
impl Default for TTVALIDATIONTESTSPARAMSEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TT_AVAILABLE: u32 = 1u32;
pub const TT_ENABLED: u32 = 2u32;
pub const TT_POLYGON_TYPE: u32 = 24u32;
pub const TT_PRIM_CSPLINE: u32 = 3u32;
pub const TT_PRIM_LINE: u32 = 1u32;
pub const TT_PRIM_QSPLINE: u32 = 2u32;
pub const TURKISH_CHARSET: FONT_CHARSET = FONT_CHARSET(162u8);
pub const VARIABLE_PITCH: FONT_PITCH = FONT_PITCH(2u8);
pub const VERTRES: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(10u32);
pub const VERTSIZE: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(6u32);
pub const VIETNAMESE_CHARSET: FONT_CHARSET = FONT_CHARSET(163u8);
pub const VREFRESH: GET_DEVICE_CAPS_INDEX = GET_DEVICE_CAPS_INDEX(116u32);
pub const VTA_BASELINE: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(24u32);
pub const VTA_BOTTOM: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(2u32);
pub const VTA_CENTER: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(6u32);
pub const VTA_LEFT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(8u32);
pub const VTA_RIGHT: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
pub const VTA_TOP: TEXT_ALIGN_OPTIONS = TEXT_ALIGN_OPTIONS(0u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCRANGE {
    pub wcLow: u16,
    pub cGlyphs: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WGLSWAP {
    pub hdc: HDC,
    pub uiFlags: u32,
}
pub const WGL_FONT_LINES: u32 = 0u32;
pub const WGL_FONT_POLYGONS: u32 = 1u32;
pub const WGL_SWAPMULTIPLE_MAX: u32 = 16u32;
pub const WGL_SWAP_MAIN_PLANE: u32 = 1u32;
pub const WGL_SWAP_OVERLAY1: u32 = 2u32;
pub const WGL_SWAP_OVERLAY10: u32 = 1024u32;
pub const WGL_SWAP_OVERLAY11: u32 = 2048u32;
pub const WGL_SWAP_OVERLAY12: u32 = 4096u32;
pub const WGL_SWAP_OVERLAY13: u32 = 8192u32;
pub const WGL_SWAP_OVERLAY14: u32 = 16384u32;
pub const WGL_SWAP_OVERLAY15: u32 = 32768u32;
pub const WGL_SWAP_OVERLAY2: u32 = 4u32;
pub const WGL_SWAP_OVERLAY3: u32 = 8u32;
pub const WGL_SWAP_OVERLAY4: u32 = 16u32;
pub const WGL_SWAP_OVERLAY5: u32 = 32u32;
pub const WGL_SWAP_OVERLAY6: u32 = 64u32;
pub const WGL_SWAP_OVERLAY7: u32 = 128u32;
pub const WGL_SWAP_OVERLAY8: u32 = 256u32;
pub const WGL_SWAP_OVERLAY9: u32 = 512u32;
pub const WGL_SWAP_UNDERLAY1: u32 = 65536u32;
pub const WGL_SWAP_UNDERLAY10: u32 = 33554432u32;
pub const WGL_SWAP_UNDERLAY11: u32 = 67108864u32;
pub const WGL_SWAP_UNDERLAY12: u32 = 134217728u32;
pub const WGL_SWAP_UNDERLAY13: u32 = 268435456u32;
pub const WGL_SWAP_UNDERLAY14: u32 = 536870912u32;
pub const WGL_SWAP_UNDERLAY15: u32 = 1073741824u32;
pub const WGL_SWAP_UNDERLAY2: u32 = 131072u32;
pub const WGL_SWAP_UNDERLAY3: u32 = 262144u32;
pub const WGL_SWAP_UNDERLAY4: u32 = 524288u32;
pub const WGL_SWAP_UNDERLAY5: u32 = 1048576u32;
pub const WGL_SWAP_UNDERLAY6: u32 = 2097152u32;
pub const WGL_SWAP_UNDERLAY7: u32 = 4194304u32;
pub const WGL_SWAP_UNDERLAY8: u32 = 8388608u32;
pub const WGL_SWAP_UNDERLAY9: u32 = 16777216u32;
pub const WHITENESS: ROP_CODE = ROP_CODE(16711778u32);
pub const WHITEONBLACK: STRETCH_BLT_MODE = STRETCH_BLT_MODE(2i32);
pub const WHITE_BRUSH: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(0i32);
pub const WHITE_PEN: GET_STOCK_OBJECT_FLAGS = GET_STOCK_OBJECT_FLAGS(6i32);
pub const WINDING: CREATE_POLYGON_RGN_MODE = CREATE_POLYGON_RGN_MODE(2i32);
pub type WRITEEMBEDPROC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *const core::ffi::c_void, param2: u32) -> u32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XFORM {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
