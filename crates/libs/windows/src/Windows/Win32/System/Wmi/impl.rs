pub trait IEnumWbemClassObject_Impl: Sized + windows_core::IUnknownImpl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self, ltimeout: i32, ucount: u32, apobjects: *mut Option<IWbemClassObject>, pureturned: *mut u32) -> windows_core::HRESULT;
    fn NextAsync(&self, ucount: u32, psink: Option<&IWbemObjectSink>) -> windows_core::HRESULT;
    fn Clone(&self) -> windows_core::Result<IEnumWbemClassObject>;
    fn Skip(&self, ltimeout: i32, ncount: u32) -> windows_core::HRESULT;
}
impl windows_core::RuntimeName for IEnumWbemClassObject {}
impl IEnumWbemClassObject_Vtbl {
    pub const fn new<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>() -> IEnumWbemClassObject_Vtbl {
        unsafe extern "system" fn Reset<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWbemClassObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Next<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut *mut core::ffi::c_void, pureturned: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWbemClassObject_Impl::Next(this, core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&ucount), core::mem::transmute_copy(&apobjects), core::mem::transmute_copy(&pureturned))
        }
        unsafe extern "system" fn NextAsync<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucount: u32, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWbemClassObject_Impl::NextAsync(this, core::mem::transmute_copy(&ucount), windows_core::from_raw_borrowed(&psink))
        }
        unsafe extern "system" fn Clone<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWbemClassObject_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ncount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWbemClassObject_Impl::Skip(this, core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&ncount))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            NextAsync: NextAsync::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWbemClassObject as windows_core::Interface>::IID
    }
}
pub trait IMofCompiler_Impl: Sized + windows_core::IUnknownImpl {
    fn CompileFile(&self, filename: &windows_core::PCWSTR, serverandnamespace: &windows_core::PCWSTR, user: &windows_core::PCWSTR, authority: &windows_core::PCWSTR, password: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
    fn CompileBuffer(&self, buffsize: i32, pbuffer: *const u8, serverandnamespace: &windows_core::PCWSTR, user: &windows_core::PCWSTR, authority: &windows_core::PCWSTR, password: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
    fn CreateBMOF(&self, textfilename: &windows_core::PCWSTR, bmoffilename: &windows_core::PCWSTR, serverandnamespace: &windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMofCompiler {}
impl IMofCompiler_Vtbl {
    pub const fn new<Identity: IMofCompiler_Impl, const OFFSET: isize>() -> IMofCompiler_Vtbl {
        unsafe extern "system" fn CompileFile<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, serverandnamespace: windows_core::PCWSTR, user: windows_core::PCWSTR, authority: windows_core::PCWSTR, password: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMofCompiler_Impl::CompileFile(this, core::mem::transmute(&filename), core::mem::transmute(&serverandnamespace), core::mem::transmute(&user), core::mem::transmute(&authority), core::mem::transmute(&password), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CompileBuffer<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: windows_core::PCWSTR, user: windows_core::PCWSTR, authority: windows_core::PCWSTR, password: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMofCompiler_Impl::CompileBuffer(this, core::mem::transmute_copy(&buffsize), core::mem::transmute_copy(&pbuffer), core::mem::transmute(&serverandnamespace), core::mem::transmute(&user), core::mem::transmute(&authority), core::mem::transmute(&password), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CreateBMOF<Identity: IMofCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textfilename: windows_core::PCWSTR, bmoffilename: windows_core::PCWSTR, serverandnamespace: windows_core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMofCompiler_Impl::CreateBMOF(this, core::mem::transmute(&textfilename), core::mem::transmute(&bmoffilename), core::mem::transmute(&serverandnamespace), core::mem::transmute_copy(&loptionflags), core::mem::transmute_copy(&lclassflags), core::mem::transmute_copy(&linstanceflags), core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompileFile: CompileFile::<Identity, OFFSET>,
            CompileBuffer: CompileBuffer::<Identity, OFFSET>,
            CreateBMOF: CreateBMOF::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMofCompiler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemDateTime_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetValue(&self, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Year(&self) -> windows_core::Result<i32>;
    fn SetYear(&self, iyear: i32) -> windows_core::Result<()>;
    fn YearSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetYearSpecified(&self, byearspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Month(&self) -> windows_core::Result<i32>;
    fn SetMonth(&self, imonth: i32) -> windows_core::Result<()>;
    fn MonthSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMonthSpecified(&self, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Day(&self) -> windows_core::Result<i32>;
    fn SetDay(&self, iday: i32) -> windows_core::Result<()>;
    fn DaySpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDaySpecified(&self, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Hours(&self) -> windows_core::Result<i32>;
    fn SetHours(&self, ihours: i32) -> windows_core::Result<()>;
    fn HoursSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHoursSpecified(&self, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Minutes(&self) -> windows_core::Result<i32>;
    fn SetMinutes(&self, iminutes: i32) -> windows_core::Result<()>;
    fn MinutesSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMinutesSpecified(&self, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Seconds(&self) -> windows_core::Result<i32>;
    fn SetSeconds(&self, iseconds: i32) -> windows_core::Result<()>;
    fn SecondsSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSecondsSpecified(&self, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Microseconds(&self) -> windows_core::Result<i32>;
    fn SetMicroseconds(&self, imicroseconds: i32) -> windows_core::Result<()>;
    fn MicrosecondsSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMicrosecondsSpecified(&self, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UTC(&self) -> windows_core::Result<i32>;
    fn SetUTC(&self, iutc: i32) -> windows_core::Result<()>;
    fn UTCSpecified(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUTCSpecified(&self, butcspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsInterval(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsInterval(&self, bisinterval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetVarDate(&self, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<f64>;
    fn SetVarDate(&self, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetFileTime(&self, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileTime(&self, strfiletime: &windows_core::BSTR, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemDateTime {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemDateTime_Vtbl {
    pub const fn new<Identity: ISWbemDateTime_Impl, const OFFSET: isize>() -> ISWbemDateTime_Vtbl {
        unsafe extern "system" fn Value<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::Value(this) {
                Ok(ok__) => {
                    strvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetValue(this, core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn Year<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iyear: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::Year(this) {
                Ok(ok__) => {
                    iyear.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYear<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iyear: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetYear(this, core::mem::transmute_copy(&iyear)).into()
        }
        unsafe extern "system" fn YearSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, byearspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::YearSpecified(this) {
                Ok(ok__) => {
                    byearspecified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYearSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, byearspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetYearSpecified(this, core::mem::transmute_copy(&byearspecified)).into()
        }
        unsafe extern "system" fn Month<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imonth: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::Month(this) {
                Ok(ok__) => {
                    imonth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonth<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imonth: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetMonth(this, core::mem::transmute_copy(&imonth)).into()
        }
        unsafe extern "system" fn MonthSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmonthspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::MonthSpecified(this) {
                Ok(ok__) => {
                    bmonthspecified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonthSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetMonthSpecified(this, core::mem::transmute_copy(&bmonthspecified)).into()
        }
        unsafe extern "system" fn Day<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iday: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::Day(this) {
                Ok(ok__) => {
                    iday.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDay<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iday: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetDay(this, core::mem::transmute_copy(&iday)).into()
        }
        unsafe extern "system" fn DaySpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdayspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::DaySpecified(this) {
                Ok(ok__) => {
                    bdayspecified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaySpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetDaySpecified(this, core::mem::transmute_copy(&bdayspecified)).into()
        }
        unsafe extern "system" fn Hours<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihours: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::Hours(this) {
                Ok(ok__) => {
                    ihours.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHours<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihours: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetHours(this, core::mem::transmute_copy(&ihours)).into()
        }
        unsafe extern "system" fn HoursSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhoursspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::HoursSpecified(this) {
                Ok(ok__) => {
                    bhoursspecified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoursSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetHoursSpecified(this, core::mem::transmute_copy(&bhoursspecified)).into()
        }
        unsafe extern "system" fn Minutes<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iminutes: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::Minutes(this) {
                Ok(ok__) => {
                    iminutes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutes<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iminutes: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetMinutes(this, core::mem::transmute_copy(&iminutes)).into()
        }
        unsafe extern "system" fn MinutesSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bminutesspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::MinutesSpecified(this) {
                Ok(ok__) => {
                    bminutesspecified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutesSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetMinutesSpecified(this, core::mem::transmute_copy(&bminutesspecified)).into()
        }
        unsafe extern "system" fn Seconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseconds: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::Seconds(this) {
                Ok(ok__) => {
                    iseconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseconds: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetSeconds(this, core::mem::transmute_copy(&iseconds)).into()
        }
        unsafe extern "system" fn SecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::SecondsSpecified(this) {
                Ok(ok__) => {
                    bsecondsspecified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetSecondsSpecified(this, core::mem::transmute_copy(&bsecondsspecified)).into()
        }
        unsafe extern "system" fn Microseconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imicroseconds: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::Microseconds(this) {
                Ok(ok__) => {
                    imicroseconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicroseconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imicroseconds: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetMicroseconds(this, core::mem::transmute_copy(&imicroseconds)).into()
        }
        unsafe extern "system" fn MicrosecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmicrosecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::MicrosecondsSpecified(this) {
                Ok(ok__) => {
                    bmicrosecondsspecified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrosecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetMicrosecondsSpecified(this, core::mem::transmute_copy(&bmicrosecondsspecified)).into()
        }
        unsafe extern "system" fn UTC<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iutc: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::UTC(this) {
                Ok(ok__) => {
                    iutc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTC<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iutc: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetUTC(this, core::mem::transmute_copy(&iutc)).into()
        }
        unsafe extern "system" fn UTCSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, butcspecified: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::UTCSpecified(this) {
                Ok(ok__) => {
                    butcspecified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, butcspecified: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetUTCSpecified(this, core::mem::transmute_copy(&butcspecified)).into()
        }
        unsafe extern "system" fn IsInterval<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisinterval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::IsInterval(this) {
                Ok(ok__) => {
                    bisinterval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInterval<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisinterval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetIsInterval(this, core::mem::transmute_copy(&bisinterval)).into()
        }
        unsafe extern "system" fn GetVarDate<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, dvardate: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::GetVarDate(this, core::mem::transmute_copy(&bislocal)) {
                Ok(ok__) => {
                    dvardate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarDate<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetVarDate(this, core::mem::transmute_copy(&dvardate), core::mem::transmute_copy(&bislocal)).into()
        }
        unsafe extern "system" fn GetFileTime<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, strfiletime: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemDateTime_Impl::GetFileTime(this, core::mem::transmute_copy(&bislocal)) {
                Ok(ok__) => {
                    strfiletime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileTime<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfiletime: core::mem::MaybeUninit<windows_core::BSTR>, bislocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemDateTime_Impl::SetFileTime(this, core::mem::transmute(&strfiletime), core::mem::transmute_copy(&bislocal)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Year: Year::<Identity, OFFSET>,
            SetYear: SetYear::<Identity, OFFSET>,
            YearSpecified: YearSpecified::<Identity, OFFSET>,
            SetYearSpecified: SetYearSpecified::<Identity, OFFSET>,
            Month: Month::<Identity, OFFSET>,
            SetMonth: SetMonth::<Identity, OFFSET>,
            MonthSpecified: MonthSpecified::<Identity, OFFSET>,
            SetMonthSpecified: SetMonthSpecified::<Identity, OFFSET>,
            Day: Day::<Identity, OFFSET>,
            SetDay: SetDay::<Identity, OFFSET>,
            DaySpecified: DaySpecified::<Identity, OFFSET>,
            SetDaySpecified: SetDaySpecified::<Identity, OFFSET>,
            Hours: Hours::<Identity, OFFSET>,
            SetHours: SetHours::<Identity, OFFSET>,
            HoursSpecified: HoursSpecified::<Identity, OFFSET>,
            SetHoursSpecified: SetHoursSpecified::<Identity, OFFSET>,
            Minutes: Minutes::<Identity, OFFSET>,
            SetMinutes: SetMinutes::<Identity, OFFSET>,
            MinutesSpecified: MinutesSpecified::<Identity, OFFSET>,
            SetMinutesSpecified: SetMinutesSpecified::<Identity, OFFSET>,
            Seconds: Seconds::<Identity, OFFSET>,
            SetSeconds: SetSeconds::<Identity, OFFSET>,
            SecondsSpecified: SecondsSpecified::<Identity, OFFSET>,
            SetSecondsSpecified: SetSecondsSpecified::<Identity, OFFSET>,
            Microseconds: Microseconds::<Identity, OFFSET>,
            SetMicroseconds: SetMicroseconds::<Identity, OFFSET>,
            MicrosecondsSpecified: MicrosecondsSpecified::<Identity, OFFSET>,
            SetMicrosecondsSpecified: SetMicrosecondsSpecified::<Identity, OFFSET>,
            UTC: UTC::<Identity, OFFSET>,
            SetUTC: SetUTC::<Identity, OFFSET>,
            UTCSpecified: UTCSpecified::<Identity, OFFSET>,
            SetUTCSpecified: SetUTCSpecified::<Identity, OFFSET>,
            IsInterval: IsInterval::<Identity, OFFSET>,
            SetIsInterval: SetIsInterval::<Identity, OFFSET>,
            GetVarDate: GetVarDate::<Identity, OFFSET>,
            SetVarDate: SetVarDate::<Identity, OFFSET>,
            GetFileTime: GetFileTime::<Identity, OFFSET>,
            SetFileTime: SetFileTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemDateTime as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemEventSource_Impl: Sized + super::Com::IDispatch_Impl {
    fn NextEvent(&self, itimeoutms: i32) -> windows_core::Result<ISWbemObject>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemEventSource {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemEventSource_Vtbl {
    pub const fn new<Identity: ISWbemEventSource_Impl, const OFFSET: isize>() -> ISWbemEventSource_Vtbl {
        unsafe extern "system" fn NextEvent<Identity: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemEventSource_Impl::NextEvent(this, core::mem::transmute_copy(&itimeoutms)) {
                Ok(ok__) => {
                    objwbemobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemEventSource_Impl::Security_(this) {
                Ok(ok__) => {
                    objwbemsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NextEvent: NextEvent::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemEventSource as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemLastError_Impl: Sized + ISWbemObject_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemLastError {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemLastError_Vtbl {
    pub const fn new<Identity: ISWbemLastError_Impl, const OFFSET: isize>() -> ISWbemLastError_Vtbl {
        Self { base__: ISWbemObject_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemLastError as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectServer(&self, strserver: &windows_core::BSTR, strnamespace: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, strauthority: &windows_core::BSTR, isecurityflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemServices>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemLocator {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemLocator_Vtbl {
    pub const fn new<Identity: ISWbemLocator_Impl, const OFFSET: isize>() -> ISWbemLocator_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: core::mem::MaybeUninit<windows_core::BSTR>, strnamespace: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, strauthority: core::mem::MaybeUninit<windows_core::BSTR>, isecurityflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemLocator_Impl::ConnectServer(this, core::mem::transmute(&strserver), core::mem::transmute(&strnamespace), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute(&strauthority), core::mem::transmute_copy(&isecurityflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemservices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemLocator_Impl::Security_(this) {
                Ok(ok__) => {
                    objwbemsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ConnectServer: ConnectServer::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemMethod_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Origin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InParameters(&self) -> windows_core::Result<ISWbemObject>;
    fn OutParameters(&self) -> windows_core::Result<ISWbemObject>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemMethod {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemMethod_Vtbl {
    pub const fn new<Identity: ISWbemMethod_Impl, const OFFSET: isize>() -> ISWbemMethod_Vtbl {
        unsafe extern "system" fn Name<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemMethod_Impl::Name(this) {
                Ok(ok__) => {
                    strname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strorigin: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemMethod_Impl::Origin(this) {
                Ok(ok__) => {
                    strorigin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InParameters<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbeminparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemMethod_Impl::InParameters(this) {
                Ok(ok__) => {
                    objwbeminparameters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutParameters<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemMethod_Impl::OutParameters(this) {
                Ok(ok__) => {
                    objwbemoutparameters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemMethod_Impl::Qualifiers_(this) {
                Ok(ok__) => {
                    objwbemqualifierset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Origin: Origin::<Identity, OFFSET>,
            InParameters: InParameters::<Identity, OFFSET>,
            OutParameters: OutParameters::<Identity, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemMethod as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemMethodSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemMethod>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemMethodSet {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemMethodSet_Vtbl {
    pub const fn new<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>() -> ISWbemMethodSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemMethodSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemmethod: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemMethodSet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemMethodSet_Impl::Count(this) {
                Ok(ok__) => {
                    icount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemMethodSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemNamedValue_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemNamedValue {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemNamedValue_Vtbl {
    pub const fn new<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>() -> ISWbemNamedValue_Vtbl {
        unsafe extern "system" fn Value<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemNamedValue_Impl::Value(this) {
                Ok(ok__) => {
                    varvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemNamedValue_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemNamedValue_Impl::Name(this) {
                Ok(ok__) => {
                    strname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemNamedValue as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemNamedValueSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemNamedValue>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, varvalue: *const super::Variant::VARIANT, iflags: i32) -> windows_core::Result<ISWbemNamedValue>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ISWbemNamedValueSet>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemNamedValueSet {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemNamedValueSet_Vtbl {
    pub const fn new<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>() -> ISWbemNamedValueSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemNamedValueSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemNamedValueSet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemnamedvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemNamedValueSet_Impl::Count(this) {
                Ok(ok__) => {
                    icount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, varvalue: *const core::mem::MaybeUninit<super::Variant::VARIANT>, iflags: i32, objwbemnamedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemNamedValueSet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&varvalue), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemnamedvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemNamedValueSet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemnamedvalueset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemNamedValueSet_Impl::Clone(this) {
                Ok(ok__) => {
                    objwbemnamedvalueset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemNamedValueSet_Impl::DeleteAll(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemNamedValueSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObject_Impl: Sized + super::Com::IDispatch_Impl {
    fn Put_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectPath>;
    fn PutAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Delete_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn DeleteAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Instances_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn InstancesAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Subclasses_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn SubclassesAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Associators_(&self, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn References_(&self, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ReferencesAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecMethod_(&self, strmethodname: &windows_core::BSTR, objwbeminparameters: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync_(&self, objwbemsink: Option<&super::Com::IDispatch>, strmethodname: &windows_core::BSTR, objwbeminparameters: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Clone_(&self) -> windows_core::Result<ISWbemObject>;
    fn GetObjectText_(&self, iflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SpawnDerivedClass_(&self, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn SpawnInstance_(&self, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn CompareTo_(&self, objwbemobject: Option<&super::Com::IDispatch>, iflags: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
    fn Properties_(&self) -> windows_core::Result<ISWbemPropertySet>;
    fn Methods_(&self) -> windows_core::Result<ISWbemMethodSet>;
    fn Derivation_(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn Path_(&self) -> windows_core::Result<ISWbemObjectPath>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemObject {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemObject_Vtbl {
    pub const fn new<Identity: ISWbemObject_Impl, const OFFSET: isize>() -> ISWbemObject_Vtbl {
        unsafe extern "system" fn Put_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Put_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObject_Impl::PutAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Delete_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObject_Impl::Delete_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn DeleteAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObject_Impl::DeleteAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Instances_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Instances_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObject_Impl::InstancesAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Subclasses_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Subclasses_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObject_Impl::SubclassesAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Associators_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strassocclass: core::mem::MaybeUninit<windows_core::BSTR>, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strresultrole: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: core::mem::MaybeUninit<windows_core::BSTR>, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Associators_(this, core::mem::transmute(&strassocclass), core::mem::transmute(&strresultclass), core::mem::transmute(&strresultrole), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredassocqualifier), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strassocclass: core::mem::MaybeUninit<windows_core::BSTR>, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strresultrole: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: core::mem::MaybeUninit<windows_core::BSTR>, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObject_Impl::AssociatorsAsync_(
                this,
                windows_core::from_raw_borrowed(&objwbemsink),
                core::mem::transmute(&strassocclass),
                core::mem::transmute(&strresultclass),
                core::mem::transmute(&strresultrole),
                core::mem::transmute(&strrole),
                core::mem::transmute_copy(&bclassesonly),
                core::mem::transmute_copy(&bschemaonly),
                core::mem::transmute(&strrequiredassocqualifier),
                core::mem::transmute(&strrequiredqualifier),
                core::mem::transmute_copy(&iflags),
                windows_core::from_raw_borrowed(&objwbemnamedvalueset),
                windows_core::from_raw_borrowed(&objwbemasynccontext),
            )
            .into()
        }
        unsafe extern "system" fn References_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::References_(this, core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObject_Impl::ReferencesAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecMethod_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::ExecMethod_(this, core::mem::transmute(&strmethodname), windows_core::from_raw_borrowed(&objwbeminparameters), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemoutparameters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObject_Impl::ExecMethodAsync_(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strmethodname), windows_core::from_raw_borrowed(&objwbeminparameters), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Clone_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Clone_(this) {
                Ok(ok__) => {
                    objwbemobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, strobjecttext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::GetObjectText_(this, core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    strobjecttext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::SpawnDerivedClass_(this, core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::SpawnInstance_(this, core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, bresult: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::CompareTo_(this, windows_core::from_raw_borrowed(&objwbemobject), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    bresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Qualifiers_(this) {
                Ok(ok__) => {
                    objwbemqualifierset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbempropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Properties_(this) {
                Ok(ok__) => {
                    objwbempropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemmethodset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Methods_(this) {
                Ok(ok__) => {
                    objwbemmethodset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Derivation_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclassnamearray: *mut core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Derivation_(this) {
                Ok(ok__) => {
                    strclassnamearray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Path_(this) {
                Ok(ok__) => {
                    objwbemobjectpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObject_Impl::Security_(this) {
                Ok(ok__) => {
                    objwbemsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Put_: Put_::<Identity, OFFSET>,
            PutAsync_: PutAsync_::<Identity, OFFSET>,
            Delete_: Delete_::<Identity, OFFSET>,
            DeleteAsync_: DeleteAsync_::<Identity, OFFSET>,
            Instances_: Instances_::<Identity, OFFSET>,
            InstancesAsync_: InstancesAsync_::<Identity, OFFSET>,
            Subclasses_: Subclasses_::<Identity, OFFSET>,
            SubclassesAsync_: SubclassesAsync_::<Identity, OFFSET>,
            Associators_: Associators_::<Identity, OFFSET>,
            AssociatorsAsync_: AssociatorsAsync_::<Identity, OFFSET>,
            References_: References_::<Identity, OFFSET>,
            ReferencesAsync_: ReferencesAsync_::<Identity, OFFSET>,
            ExecMethod_: ExecMethod_::<Identity, OFFSET>,
            ExecMethodAsync_: ExecMethodAsync_::<Identity, OFFSET>,
            Clone_: Clone_::<Identity, OFFSET>,
            GetObjectText_: GetObjectText_::<Identity, OFFSET>,
            SpawnDerivedClass_: SpawnDerivedClass_::<Identity, OFFSET>,
            SpawnInstance_: SpawnInstance_::<Identity, OFFSET>,
            CompareTo_: CompareTo_::<Identity, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, OFFSET>,
            Properties_: Properties_::<Identity, OFFSET>,
            Methods_: Methods_::<Identity, OFFSET>,
            Derivation_: Derivation_::<Identity, OFFSET>,
            Path_: Path_::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObject as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectEx_Impl: Sized + ISWbemObject_Impl {
    fn Refresh_(&self, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn SystemProperties_(&self) -> windows_core::Result<ISWbemPropertySet>;
    fn GetText_(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<windows_core::BSTR>;
    fn SetFromText_(&self, bstext: &windows_core::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemObjectEx {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemObjectEx_Vtbl {
    pub const fn new<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>() -> ISWbemObjectEx_Vtbl {
        unsafe extern "system" fn Refresh_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectEx_Impl::Refresh_(this, core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn SystemProperties_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbempropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectEx_Impl::SystemProperties_(this) {
                Ok(ok__) => {
                    objwbempropertyset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, bstext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectEx_Impl::GetText_(this, core::mem::transmute_copy(&iobjecttextformat), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    bstext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromText_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstext: core::mem::MaybeUninit<windows_core::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectEx_Impl::SetFromText_(this, core::mem::transmute(&bstext), core::mem::transmute_copy(&iobjecttextformat), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into()
        }
        Self {
            base__: ISWbemObject_Vtbl::new::<Identity, OFFSET>(),
            Refresh_: Refresh_::<Identity, OFFSET>,
            SystemProperties_: SystemProperties_::<Identity, OFFSET>,
            GetText_: GetText_::<Identity, OFFSET>,
            SetFromText_: SetFromText_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectPath_Impl: Sized + super::Com::IDispatch_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, strpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RelPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRelPath(&self, strrelpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Server(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServer(&self, strserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Namespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNamespace(&self, strnamespace: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ParentNamespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, strdisplayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Class(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClass(&self, strclass: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsClass(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAsClass(&self) -> windows_core::Result<()>;
    fn IsSingleton(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAsSingleton(&self) -> windows_core::Result<()>;
    fn Keys(&self) -> windows_core::Result<ISWbemNamedValueSet>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
    fn Locale(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocale(&self, strlocale: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Authority(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthority(&self, strauthority: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemObjectPath {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemObjectPath_Vtbl {
    pub const fn new<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>() -> ISWbemObjectPath_Vtbl {
        unsafe extern "system" fn Path<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::Path(this) {
                Ok(ok__) => {
                    strpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetPath(this, core::mem::transmute(&strpath)).into()
        }
        unsafe extern "system" fn RelPath<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrelpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::RelPath(this) {
                Ok(ok__) => {
                    strrelpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelPath<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrelpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetRelPath(this, core::mem::transmute(&strrelpath)).into()
        }
        unsafe extern "system" fn Server<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::Server(this) {
                Ok(ok__) => {
                    strserver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetServer(this, core::mem::transmute(&strserver)).into()
        }
        unsafe extern "system" fn Namespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::Namespace(this) {
                Ok(ok__) => {
                    strnamespace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetNamespace(this, core::mem::transmute(&strnamespace)).into()
        }
        unsafe extern "system" fn ParentNamespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strparentnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::ParentNamespace(this) {
                Ok(ok__) => {
                    strparentnamespace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::DisplayName(this) {
                Ok(ok__) => {
                    strdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetDisplayName(this, core::mem::transmute(&strdisplayname)).into()
        }
        unsafe extern "system" fn Class<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::Class(this) {
                Ok(ok__) => {
                    strclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClass<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetClass(this, core::mem::transmute(&strclass)).into()
        }
        unsafe extern "system" fn IsClass<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisclass: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::IsClass(this) {
                Ok(ok__) => {
                    bisclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsClass<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetAsClass(this).into()
        }
        unsafe extern "system" fn IsSingleton<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bissingleton: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::IsSingleton(this) {
                Ok(ok__) => {
                    bissingleton.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsSingleton<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetAsSingleton(this).into()
        }
        unsafe extern "system" fn Keys<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemnamedvalueset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::Keys(this) {
                Ok(ok__) => {
                    objwbemnamedvalueset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::Security_(this) {
                Ok(ok__) => {
                    objwbemsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locale<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strlocale: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::Locale(this) {
                Ok(ok__) => {
                    strlocale.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocale<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strlocale: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetLocale(this, core::mem::transmute(&strlocale)).into()
        }
        unsafe extern "system" fn Authority<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strauthority: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectPath_Impl::Authority(this) {
                Ok(ok__) => {
                    strauthority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthority<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strauthority: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemObjectPath_Impl::SetAuthority(this, core::mem::transmute(&strauthority)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            RelPath: RelPath::<Identity, OFFSET>,
            SetRelPath: SetRelPath::<Identity, OFFSET>,
            Server: Server::<Identity, OFFSET>,
            SetServer: SetServer::<Identity, OFFSET>,
            Namespace: Namespace::<Identity, OFFSET>,
            SetNamespace: SetNamespace::<Identity, OFFSET>,
            ParentNamespace: ParentNamespace::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            Class: Class::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            IsClass: IsClass::<Identity, OFFSET>,
            SetAsClass: SetAsClass::<Identity, OFFSET>,
            IsSingleton: IsSingleton::<Identity, OFFSET>,
            SetAsSingleton: SetAsSingleton::<Identity, OFFSET>,
            Keys: Keys::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
            Locale: Locale::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            Authority: Authority::<Identity, OFFSET>,
            SetAuthority: SetAuthority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectPath as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemObjectSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strobjectpath: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
    fn ItemIndex(&self, lindex: i32) -> windows_core::Result<ISWbemObject>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemObjectSet {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemObjectSet_Vtbl {
    pub const fn new<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>() -> ISWbemObjectSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectSet_Impl::Item(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectSet_Impl::Count(this) {
                Ok(ok__) => {
                    icount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectSet_Impl::Security_(this) {
                Ok(ok__) => {
                    objwbemsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemIndex<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemObjectSet_Impl::ItemIndex(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    objwbemobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
            ItemIndex: ItemIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPrivilege_Impl: Sized + super::Com::IDispatch_Impl {
    fn IsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsEnabled(&self, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Identifier(&self) -> windows_core::Result<WbemPrivilegeEnum>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemPrivilege {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemPrivilege_Vtbl {
    pub const fn new<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>() -> ISWbemPrivilege_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilege_Impl::IsEnabled(this) {
                Ok(ok__) => {
                    bisenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemPrivilege_Impl::SetIsEnabled(this, core::mem::transmute_copy(&bisenabled)).into()
        }
        unsafe extern "system" fn Name<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilege_Impl::Name(this) {
                Ok(ok__) => {
                    strdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilege_Impl::DisplayName(this) {
                Ok(ok__) => {
                    strdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilege_Impl::Identifier(this) {
                Ok(ok__) => {
                    iprivilege.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            Identifier: Identifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPrivilege as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPrivilegeSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<ISWbemPrivilege>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege>;
    fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
    fn AddAsString(&self, strprivilege: &windows_core::BSTR, bisenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemPrivilegeSet {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemPrivilegeSet_Vtbl {
    pub const fn new<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>() -> ISWbemPrivilegeSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilegeSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilegeSet_Impl::Item(this, core::mem::transmute_copy(&iprivilege)) {
                Ok(ok__) => {
                    objwbemprivilege.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilegeSet_Impl::Count(this) {
                Ok(ok__) => {
                    icount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilegeSet_Impl::Add(this, core::mem::transmute_copy(&iprivilege), core::mem::transmute_copy(&bisenabled)) {
                Ok(ok__) => {
                    objwbemprivilege.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemPrivilegeSet_Impl::Remove(this, core::mem::transmute_copy(&iprivilege)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemPrivilegeSet_Impl::DeleteAll(this).into()
        }
        unsafe extern "system" fn AddAsString<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprivilege: core::mem::MaybeUninit<windows_core::BSTR>, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPrivilegeSet_Impl::AddAsString(this, core::mem::transmute(&strprivilege), core::mem::transmute_copy(&bisenabled)) {
                Ok(ok__) => {
                    objwbemprivilege.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
            AddAsString: AddAsString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPrivilegeSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Origin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CIMType(&self) -> windows_core::Result<WbemCimtypeEnum>;
    fn Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
    fn IsArray(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemProperty {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemProperty_Vtbl {
    pub const fn new<Identity: ISWbemProperty_Impl, const OFFSET: isize>() -> ISWbemProperty_Vtbl {
        unsafe extern "system" fn Value<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemProperty_Impl::Value(this) {
                Ok(ok__) => {
                    varvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemProperty_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemProperty_Impl::Name(this) {
                Ok(ok__) => {
                    strname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemProperty_Impl::IsLocal(this) {
                Ok(ok__) => {
                    bislocal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strorigin: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemProperty_Impl::Origin(this) {
                Ok(ok__) => {
                    strorigin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CIMType<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemProperty_Impl::CIMType(this) {
                Ok(ok__) => {
                    icimtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemProperty_Impl::Qualifiers_(this) {
                Ok(ok__) => {
                    objwbemqualifierset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArray<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisarray: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemProperty_Impl::IsArray(this) {
                Ok(ok__) => {
                    bisarray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            Origin: Origin::<Identity, OFFSET>,
            CIMType: CIMType::<Identity, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, OFFSET>,
            IsArray: IsArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemProperty as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemPropertySet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemProperty>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemProperty>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemPropertySet {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemPropertySet_Vtbl {
    pub const fn new<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>() -> ISWbemPropertySet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPropertySet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPropertySet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPropertySet_Impl::Count(this) {
                Ok(ok__) => {
                    icount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemPropertySet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&icimtype), core::mem::transmute_copy(&bisarray), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemPropertySet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPropertySet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemQualifier_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn PropagatesToSubclass(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPropagatesToSubclass(&self, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PropagatesToInstance(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPropagatesToInstance(&self, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsOverridable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsOverridable(&self, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsAmended(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemQualifier {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemQualifier_Vtbl {
    pub const fn new<Identity: ISWbemQualifier_Impl, const OFFSET: isize>() -> ISWbemQualifier_Vtbl {
        unsafe extern "system" fn Value<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifier_Impl::Value(this) {
                Ok(ok__) => {
                    varvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemQualifier_Impl::SetValue(this, core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifier_Impl::Name(this) {
                Ok(ok__) => {
                    strname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifier_Impl::IsLocal(this) {
                Ok(ok__) => {
                    bislocal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropagatesToSubclass<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestosubclass: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifier_Impl::PropagatesToSubclass(this) {
                Ok(ok__) => {
                    bpropagatestosubclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToSubclass<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemQualifier_Impl::SetPropagatesToSubclass(this, core::mem::transmute_copy(&bpropagatestosubclass)).into()
        }
        unsafe extern "system" fn PropagatesToInstance<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestoinstance: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifier_Impl::PropagatesToInstance(this) {
                Ok(ok__) => {
                    bpropagatestoinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToInstance<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemQualifier_Impl::SetPropagatesToInstance(this, core::mem::transmute_copy(&bpropagatestoinstance)).into()
        }
        unsafe extern "system" fn IsOverridable<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisoverridable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifier_Impl::IsOverridable(this) {
                Ok(ok__) => {
                    bisoverridable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverridable<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemQualifier_Impl::SetIsOverridable(this, core::mem::transmute_copy(&bisoverridable)).into()
        }
        unsafe extern "system" fn IsAmended<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisamended: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifier_Impl::IsAmended(this) {
                Ok(ok__) => {
                    bisamended.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            PropagatesToSubclass: PropagatesToSubclass::<Identity, OFFSET>,
            SetPropagatesToSubclass: SetPropagatesToSubclass::<Identity, OFFSET>,
            PropagatesToInstance: PropagatesToInstance::<Identity, OFFSET>,
            SetPropagatesToInstance: SetPropagatesToInstance::<Identity, OFFSET>,
            IsOverridable: IsOverridable::<Identity, OFFSET>,
            SetIsOverridable: SetIsOverridable::<Identity, OFFSET>,
            IsAmended: IsAmended::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemQualifier as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemQualifierSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, name: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemQualifier>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, varval: *const super::Variant::VARIANT, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemQualifier>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemQualifierSet {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemQualifierSet_Vtbl {
    pub const fn new<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>() -> ISWbemQualifierSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifierSet_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemqualifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifierSet_Impl::Item(this, core::mem::transmute(&name), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemqualifier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifierSet_Impl::Count(this) {
                Ok(ok__) => {
                    icount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, varval: *const core::mem::MaybeUninit<super::Variant::VARIANT>, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemqualifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemQualifierSet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&varval), core::mem::transmute_copy(&bpropagatestosubclass), core::mem::transmute_copy(&bpropagatestoinstance), core::mem::transmute_copy(&bisoverridable), core::mem::transmute_copy(&iflags)) {
                Ok(ok__) => {
                    objwbemqualifier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemQualifierSet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemQualifierSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemRefreshableItem_Impl: Sized + super::Com::IDispatch_Impl {
    fn Index(&self) -> windows_core::Result<i32>;
    fn Refresher(&self) -> windows_core::Result<ISWbemRefresher>;
    fn IsSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Object(&self) -> windows_core::Result<ISWbemObjectEx>;
    fn ObjectSet(&self) -> windows_core::Result<ISWbemObjectSet>;
    fn Remove(&self, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemRefreshableItem {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemRefreshableItem_Vtbl {
    pub const fn new<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>() -> ISWbemRefreshableItem_Vtbl {
        unsafe extern "system" fn Index<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefreshableItem_Impl::Index(this) {
                Ok(ok__) => {
                    iindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresher<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemrefresher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefreshableItem_Impl::Refresher(this) {
                Ok(ok__) => {
                    objwbemrefresher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSet<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisset: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefreshableItem_Impl::IsSet(this) {
                Ok(ok__) => {
                    bisset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Object<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefreshableItem_Impl::Object(this) {
                Ok(ok__) => {
                    objwbemobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectSet<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefreshableItem_Impl::ObjectSet(this) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemRefreshableItem_Impl::Remove(this, core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, OFFSET>,
            Refresher: Refresher::<Identity, OFFSET>,
            IsSet: IsSet::<Identity, OFFSET>,
            Object: Object::<Identity, OFFSET>,
            ObjectSet: ObjectSet::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemRefreshableItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemRefresher_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, iindex: i32) -> windows_core::Result<ISWbemRefreshableItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, objwbemservices: Option<&ISWbemServicesEx>, bsinstancepath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemRefreshableItem>;
    fn AddEnum(&self, objwbemservices: Option<&ISWbemServicesEx>, bsclassname: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemRefreshableItem>;
    fn Remove(&self, iindex: i32, iflags: i32) -> windows_core::Result<()>;
    fn Refresh(&self, iflags: i32) -> windows_core::Result<()>;
    fn AutoReconnect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoReconnect(&self, bcount: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemRefresher {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemRefresher_Vtbl {
    pub const fn new<Identity: ISWbemRefresher_Impl, const OFFSET: isize>() -> ISWbemRefresher_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefresher_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefresher_Impl::Item(this, core::mem::transmute_copy(&iindex)) {
                Ok(ok__) => {
                    objwbemrefreshableitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefresher_Impl::Count(this) {
                Ok(ok__) => {
                    icount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemservices: *mut core::ffi::c_void, bsinstancepath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefresher_Impl::Add(this, windows_core::from_raw_borrowed(&objwbemservices), core::mem::transmute(&bsinstancepath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemrefreshableitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnum<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemservices: *mut core::ffi::c_void, bsclassname: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefresher_Impl::AddEnum(this, windows_core::from_raw_borrowed(&objwbemservices), core::mem::transmute(&bsclassname), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemrefreshableitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, iflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemRefresher_Impl::Remove(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemRefresher_Impl::Refresh(this, core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn AutoReconnect<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcount: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemRefresher_Impl::AutoReconnect(this) {
                Ok(ok__) => {
                    bcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnect<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcount: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemRefresher_Impl::SetAutoReconnect(this, core::mem::transmute_copy(&bcount)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemRefresher_Impl::DeleteAll(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            AddEnum: AddEnum::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            AutoReconnect: AutoReconnect::<Identity, OFFSET>,
            SetAutoReconnect: SetAutoReconnect::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemRefresher as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSecurity_Impl: Sized + super::Com::IDispatch_Impl {
    fn ImpersonationLevel(&self) -> windows_core::Result<WbemImpersonationLevelEnum>;
    fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::Result<()>;
    fn AuthenticationLevel(&self) -> windows_core::Result<WbemAuthenticationLevelEnum>;
    fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::Result<()>;
    fn Privileges(&self) -> windows_core::Result<ISWbemPrivilegeSet>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemSecurity {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemSecurity_Vtbl {
    pub const fn new<Identity: ISWbemSecurity_Impl, const OFFSET: isize>() -> ISWbemSecurity_Vtbl {
        unsafe extern "system" fn ImpersonationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemSecurity_Impl::ImpersonationLevel(this) {
                Ok(ok__) => {
                    iimpersonationlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImpersonationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemSecurity_Impl::SetImpersonationLevel(this, core::mem::transmute_copy(&iimpersonationlevel)).into()
        }
        unsafe extern "system" fn AuthenticationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemSecurity_Impl::AuthenticationLevel(this) {
                Ok(ok__) => {
                    iauthenticationlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemSecurity_Impl::SetAuthenticationLevel(this, core::mem::transmute_copy(&iauthenticationlevel)).into()
        }
        unsafe extern "system" fn Privileges<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemprivilegeset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemSecurity_Impl::Privileges(this) {
                Ok(ok__) => {
                    objwbemprivilegeset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ImpersonationLevel: ImpersonationLevel::<Identity, OFFSET>,
            SetImpersonationLevel: SetImpersonationLevel::<Identity, OFFSET>,
            AuthenticationLevel: AuthenticationLevel::<Identity, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, OFFSET>,
            Privileges: Privileges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSecurity as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemServices_Impl: Sized + super::Com::IDispatch_Impl {
    fn Get(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn GetAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Delete(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn DeleteAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn InstancesOf(&self, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn InstancesOfAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn SubclassesOf(&self, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn SubclassesOfAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecQuery(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ExecQueryAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, lflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn AssociatorsOf(&self, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsOfAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ReferencesTo(&self, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ReferencesToAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecNotificationQuery(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemEventSource>;
    fn ExecNotificationQueryAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync(&self, objwbemsink: Option<&super::Com::IDispatch>, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: Option<&super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemServices {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemServices_Vtbl {
    pub const fn new<Identity: ISWbemServices_Impl, const OFFSET: isize>() -> ISWbemServices_Vtbl {
        unsafe extern "system" fn Get<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::Get(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::GetAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Delete<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::Delete(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn DeleteAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::DeleteAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn InstancesOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::InstancesOf(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::InstancesOfAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strclass), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn SubclassesOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::SubclassesOf(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::SubclassesOfAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecQuery<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquery: core::mem::MaybeUninit<windows_core::BSTR>, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::ExecQuery(this, core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strquery: core::mem::MaybeUninit<windows_core::BSTR>, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::ExecQueryAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn AssociatorsOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strassocclass: core::mem::MaybeUninit<windows_core::BSTR>, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strresultrole: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: core::mem::MaybeUninit<windows_core::BSTR>, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::AssociatorsOf(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strassocclass), core::mem::transmute(&strresultclass), core::mem::transmute(&strresultrole), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredassocqualifier), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            objwbemsink: *mut core::ffi::c_void,
            strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>,
            strassocclass: core::mem::MaybeUninit<windows_core::BSTR>,
            strresultclass: core::mem::MaybeUninit<windows_core::BSTR>,
            strresultrole: core::mem::MaybeUninit<windows_core::BSTR>,
            strrole: core::mem::MaybeUninit<windows_core::BSTR>,
            bclassesonly: super::super::Foundation::VARIANT_BOOL,
            bschemaonly: super::super::Foundation::VARIANT_BOOL,
            strrequiredassocqualifier: core::mem::MaybeUninit<windows_core::BSTR>,
            strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut core::ffi::c_void,
            objwbemasynccontext: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::AssociatorsOfAsync(
                this,
                windows_core::from_raw_borrowed(&objwbemsink),
                core::mem::transmute(&strobjectpath),
                core::mem::transmute(&strassocclass),
                core::mem::transmute(&strresultclass),
                core::mem::transmute(&strresultrole),
                core::mem::transmute(&strrole),
                core::mem::transmute_copy(&bclassesonly),
                core::mem::transmute_copy(&bschemaonly),
                core::mem::transmute(&strrequiredassocqualifier),
                core::mem::transmute(&strrequiredqualifier),
                core::mem::transmute_copy(&iflags),
                windows_core::from_raw_borrowed(&objwbemnamedvalueset),
                windows_core::from_raw_borrowed(&objwbemasynccontext),
            )
            .into()
        }
        unsafe extern "system" fn ReferencesTo<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::ReferencesTo(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesToAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strresultclass: core::mem::MaybeUninit<windows_core::BSTR>, strrole: core::mem::MaybeUninit<windows_core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::ReferencesToAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquery: core::mem::MaybeUninit<windows_core::BSTR>, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemeventsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::ExecNotificationQuery(this, core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemeventsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strquery: core::mem::MaybeUninit<windows_core::BSTR>, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::ExecNotificationQueryAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecMethod<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::ExecMethod(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), windows_core::from_raw_borrowed(&objwbeminparameters), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemoutparameters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServices_Impl::ExecMethodAsync(this, windows_core::from_raw_borrowed(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), windows_core::from_raw_borrowed(&objwbeminparameters), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Security_<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServices_Impl::Security_(this) {
                Ok(ok__) => {
                    objwbemsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            GetAsync: GetAsync::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAsync: DeleteAsync::<Identity, OFFSET>,
            InstancesOf: InstancesOf::<Identity, OFFSET>,
            InstancesOfAsync: InstancesOfAsync::<Identity, OFFSET>,
            SubclassesOf: SubclassesOf::<Identity, OFFSET>,
            SubclassesOfAsync: SubclassesOfAsync::<Identity, OFFSET>,
            ExecQuery: ExecQuery::<Identity, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, OFFSET>,
            AssociatorsOf: AssociatorsOf::<Identity, OFFSET>,
            AssociatorsOfAsync: AssociatorsOfAsync::<Identity, OFFSET>,
            ReferencesTo: ReferencesTo::<Identity, OFFSET>,
            ReferencesToAsync: ReferencesToAsync::<Identity, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, OFFSET>,
            ExecMethod: ExecMethod::<Identity, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, OFFSET>,
            Security_: Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemServices as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemServicesEx_Impl: Sized + ISWbemServices_Impl {
    fn Put(&self, objwbemobject: Option<&ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>) -> windows_core::Result<ISWbemObjectPath>;
    fn PutAsync(&self, objwbemsink: Option<&ISWbemSink>, objwbemobject: Option<&ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: Option<&super::Com::IDispatch>, objwbemasynccontext: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemServicesEx {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemServicesEx_Vtbl {
    pub const fn new<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>() -> ISWbemServicesEx_Vtbl {
        unsafe extern "system" fn Put<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISWbemServicesEx_Impl::Put(this, windows_core::from_raw_borrowed(&objwbemobject), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset)) {
                Ok(ok__) => {
                    objwbemobjectpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemServicesEx_Impl::PutAsync(this, windows_core::from_raw_borrowed(&objwbemsink), windows_core::from_raw_borrowed(&objwbemobject), core::mem::transmute_copy(&iflags), windows_core::from_raw_borrowed(&objwbemnamedvalueset), windows_core::from_raw_borrowed(&objwbemasynccontext)).into()
        }
        Self { base__: ISWbemServices_Vtbl::new::<Identity, OFFSET>(), Put: Put::<Identity, OFFSET>, PutAsync: PutAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemServicesEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSink_Impl: Sized + super::Com::IDispatch_Impl {
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemSink {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemSink_Vtbl {
    pub const fn new<Identity: ISWbemSink_Impl, const OFFSET: isize>() -> ISWbemSink_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ISWbemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISWbemSink_Impl::Cancel(this).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSink as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISWbemSinkEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISWbemSinkEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISWbemSinkEvents_Vtbl {
    pub const fn new<Identity: ISWbemSinkEvents_Impl, const OFFSET: isize>() -> ISWbemSinkEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSinkEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IUnsecuredApartment_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateObjectStub(&self, pobject: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IUnsecuredApartment {}
impl IUnsecuredApartment_Vtbl {
    pub const fn new<Identity: IUnsecuredApartment_Impl, const OFFSET: isize>() -> IUnsecuredApartment_Vtbl {
        unsafe extern "system" fn CreateObjectStub<Identity: IUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUnsecuredApartment_Impl::CreateObjectStub(this, windows_core::from_raw_borrowed(&pobject)) {
                Ok(ok__) => {
                    ppstub.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObjectStub: CreateObjectStub::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnsecuredApartment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMIExtension_Impl: Sized + super::Com::IDispatch_Impl {
    fn WMIObjectPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetWMIObject(&self) -> windows_core::Result<ISWbemObject>;
    fn GetWMIServices(&self) -> windows_core::Result<ISWbemServices>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWMIExtension {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWMIExtension_Vtbl {
    pub const fn new<Identity: IWMIExtension_Impl, const OFFSET: isize>() -> IWMIExtension_Vtbl {
        unsafe extern "system" fn WMIObjectPath<Identity: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strwmiobjectpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMIExtension_Impl::WMIObjectPath(this) {
                Ok(ok__) => {
                    strwmiobjectpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIObject<Identity: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwmiobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMIExtension_Impl::GetWMIObject(this) {
                Ok(ok__) => {
                    objwmiobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIServices<Identity: IWMIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwmiservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMIExtension_Impl::GetWMIServices(this) {
                Ok(ok__) => {
                    objwmiservices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WMIObjectPath: WMIObjectPath::<Identity, OFFSET>,
            GetWMIObject: GetWMIObject::<Identity, OFFSET>,
            GetWMIServices: GetWMIServices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIExtension as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWbemAddressResolution_Impl: Sized + windows_core::IUnknownImpl {
    fn Resolve(&self, wsznamespacepath: &windows_core::PCWSTR, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemAddressResolution {}
impl IWbemAddressResolution_Vtbl {
    pub const fn new<Identity: IWbemAddressResolution_Impl, const OFFSET: isize>() -> IWbemAddressResolution_Vtbl {
        unsafe extern "system" fn Resolve<Identity: IWbemAddressResolution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznamespacepath: windows_core::PCWSTR, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemAddressResolution_Impl::Resolve(this, core::mem::transmute(&wsznamespacepath), core::mem::transmute_copy(&wszaddresstype), core::mem::transmute_copy(&pdwaddresslength), core::mem::transmute_copy(&pabbinaryaddress)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemAddressResolution as windows_core::Interface>::IID
    }
}
pub trait IWbemBackupRestore_Impl: Sized + windows_core::IUnknownImpl {
    fn Backup(&self, strbackuptofile: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
    fn Restore(&self, strrestorefromfile: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemBackupRestore {}
impl IWbemBackupRestore_Vtbl {
    pub const fn new<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>() -> IWbemBackupRestore_Vtbl {
        unsafe extern "system" fn Backup<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbackuptofile: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemBackupRestore_Impl::Backup(this, core::mem::transmute(&strbackuptofile), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Restore<Identity: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrestorefromfile: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemBackupRestore_Impl::Restore(this, core::mem::transmute(&strrestorefromfile), core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Backup: Backup::<Identity, OFFSET>, Restore: Restore::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemBackupRestore as windows_core::Interface>::IID
    }
}
pub trait IWbemBackupRestoreEx_Impl: Sized + IWbemBackupRestore_Impl {
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemBackupRestoreEx {}
impl IWbemBackupRestoreEx_Vtbl {
    pub const fn new<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>() -> IWbemBackupRestoreEx_Vtbl {
        unsafe extern "system" fn Pause<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemBackupRestoreEx_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemBackupRestoreEx_Impl::Resume(this).into()
        }
        Self { base__: IWbemBackupRestore_Vtbl::new::<Identity, OFFSET>(), Pause: Pause::<Identity, OFFSET>, Resume: Resume::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemBackupRestoreEx as windows_core::Interface>::IID || iid == &<IWbemBackupRestore as windows_core::Interface>::IID
    }
}
pub trait IWbemCallResult_Impl: Sized + windows_core::IUnknownImpl {
    fn GetResultObject(&self, ltimeout: i32) -> windows_core::Result<IWbemClassObject>;
    fn GetResultString(&self, ltimeout: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetResultServices(&self, ltimeout: i32) -> windows_core::Result<IWbemServices>;
    fn GetCallStatus(&self, ltimeout: i32) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IWbemCallResult {}
impl IWbemCallResult_Vtbl {
    pub const fn new<Identity: IWbemCallResult_Impl, const OFFSET: isize>() -> IWbemCallResult_Vtbl {
        unsafe extern "system" fn GetResultObject<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ppresultobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemCallResult_Impl::GetResultObject(this, core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    ppresultobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultString<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemCallResult_Impl::GetResultString(this, core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    pstrresultstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultServices<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, ppservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemCallResult_Impl::GetResultServices(this, core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    ppservices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallStatus<Identity: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemCallResult_Impl::GetCallStatus(this, core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    plstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResultObject: GetResultObject::<Identity, OFFSET>,
            GetResultString: GetResultString::<Identity, OFFSET>,
            GetResultServices: GetResultServices::<Identity, OFFSET>,
            GetCallStatus: GetCallStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemCallResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemClassObject_Impl: Sized + windows_core::IUnknownImpl {
    fn GetQualifierSet(&self) -> windows_core::Result<IWbemQualifierSet>;
    fn Get(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::Result<()>;
    fn Put(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *const super::Variant::VARIANT, r#type: i32) -> windows_core::Result<()>;
    fn Delete(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNames(&self, wszqualifiername: &windows_core::PCWSTR, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const super::Variant::VARIANT) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lenumflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, strname: *mut windows_core::BSTR, pval: *mut super::Variant::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
    fn GetPropertyQualifierSet(&self, wszproperty: &windows_core::PCWSTR) -> windows_core::Result<IWbemQualifierSet>;
    fn Clone(&self) -> windows_core::Result<IWbemClassObject>;
    fn GetObjectText(&self, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SpawnDerivedClass(&self, lflags: i32) -> windows_core::Result<IWbemClassObject>;
    fn SpawnInstance(&self, lflags: i32) -> windows_core::Result<IWbemClassObject>;
    fn CompareTo(&self, lflags: WBEM_COMPARISON_FLAG, pcompareto: Option<&IWbemClassObject>) -> windows_core::Result<()>;
    fn GetPropertyOrigin(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn InheritsFrom(&self, strancestor: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetMethod(&self, wszname: &windows_core::PCWSTR, lflags: i32, ppinsignature: *mut Option<IWbemClassObject>, ppoutsignature: *mut Option<IWbemClassObject>) -> windows_core::Result<()>;
    fn PutMethod(&self, wszname: &windows_core::PCWSTR, lflags: i32, pinsignature: Option<&IWbemClassObject>, poutsignature: Option<&IWbemClassObject>) -> windows_core::Result<()>;
    fn DeleteMethod(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn BeginMethodEnumeration(&self, lenumflags: i32) -> windows_core::Result<()>;
    fn NextMethod(&self, lflags: i32, pstrname: *mut windows_core::BSTR, ppinsignature: *mut Option<IWbemClassObject>, ppoutsignature: *mut Option<IWbemClassObject>) -> windows_core::Result<()>;
    fn EndMethodEnumeration(&self) -> windows_core::Result<()>;
    fn GetMethodQualifierSet(&self, wszmethod: &windows_core::PCWSTR) -> windows_core::Result<IWbemQualifierSet>;
    fn GetMethodOrigin(&self, wszmethodname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemClassObject {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemClassObject_Vtbl {
    pub const fn new<Identity: IWbemClassObject_Impl, const OFFSET: isize>() -> IWbemClassObject_Vtbl {
        unsafe extern "system" fn GetQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::GetQualifierSet(this) {
                Ok(ok__) => {
                    ppqualset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *mut core::mem::MaybeUninit<super::Variant::VARIANT>, ptype: *mut i32, plflavor: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::Get(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn Put<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *const core::mem::MaybeUninit<super::Variant::VARIANT>, r#type: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::Put(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Delete<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::Delete(this, core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn GetNames<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszqualifiername: windows_core::PCWSTR, lflags: WBEM_CONDITION_FLAG_TYPE, pqualifierval: *const core::mem::MaybeUninit<super::Variant::VARIANT>, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::GetNames(this, core::mem::transmute(&wszqualifiername), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pqualifierval)) {
                Ok(ok__) => {
                    pnames.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lenumflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lenumflags)).into()
        }
        unsafe extern "system" fn Next<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<super::Variant::VARIANT>, ptype: *mut i32, plflavor: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&strname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::EndEnumeration(this).into()
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszproperty: windows_core::PCWSTR, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::GetPropertyQualifierSet(this, core::mem::transmute(&wszproperty)) {
                Ok(ok__) => {
                    ppqualset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::Clone(this) {
                Ok(ok__) => {
                    ppcopy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrobjecttext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::GetObjectText(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    pstrobjecttext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppnewclass: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::SpawnDerivedClass(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppnewclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppnewinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::SpawnInstance(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppnewinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: WBEM_COMPARISON_FLAG, pcompareto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::CompareTo(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcompareto)).into()
        }
        unsafe extern "system" fn GetPropertyOrigin<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, pstrclassname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::GetPropertyOrigin(this, core::mem::transmute(&wszname)) {
                Ok(ok__) => {
                    pstrclassname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InheritsFrom<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strancestor: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::InheritsFrom(this, core::mem::transmute(&strancestor)).into()
        }
        unsafe extern "system" fn GetMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, ppinsignature: *mut *mut core::ffi::c_void, ppoutsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::GetMethod(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ppinsignature), core::mem::transmute_copy(&ppoutsignature)).into()
        }
        unsafe extern "system" fn PutMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pinsignature: *mut core::ffi::c_void, poutsignature: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::PutMethod(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pinsignature), windows_core::from_raw_borrowed(&poutsignature)).into()
        }
        unsafe extern "system" fn DeleteMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::DeleteMethod(this, core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn BeginMethodEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lenumflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::BeginMethodEnumeration(this, core::mem::transmute_copy(&lenumflags)).into()
        }
        unsafe extern "system" fn NextMethod<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppinsignature: *mut *mut core::ffi::c_void, ppoutsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::NextMethod(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&ppinsignature), core::mem::transmute_copy(&ppoutsignature)).into()
        }
        unsafe extern "system" fn EndMethodEnumeration<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClassObject_Impl::EndMethodEnumeration(this).into()
        }
        unsafe extern "system" fn GetMethodQualifierSet<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethod: windows_core::PCWSTR, ppqualset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::GetMethodQualifierSet(this, core::mem::transmute(&wszmethod)) {
                Ok(ok__) => {
                    ppqualset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodOrigin<Identity: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethodname: windows_core::PCWSTR, pstrclassname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClassObject_Impl::GetMethodOrigin(this, core::mem::transmute(&wszmethodname)) {
                Ok(ok__) => {
                    pstrclassname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetQualifierSet: GetQualifierSet::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
            GetPropertyQualifierSet: GetPropertyQualifierSet::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetObjectText: GetObjectText::<Identity, OFFSET>,
            SpawnDerivedClass: SpawnDerivedClass::<Identity, OFFSET>,
            SpawnInstance: SpawnInstance::<Identity, OFFSET>,
            CompareTo: CompareTo::<Identity, OFFSET>,
            GetPropertyOrigin: GetPropertyOrigin::<Identity, OFFSET>,
            InheritsFrom: InheritsFrom::<Identity, OFFSET>,
            GetMethod: GetMethod::<Identity, OFFSET>,
            PutMethod: PutMethod::<Identity, OFFSET>,
            DeleteMethod: DeleteMethod::<Identity, OFFSET>,
            BeginMethodEnumeration: BeginMethodEnumeration::<Identity, OFFSET>,
            NextMethod: NextMethod::<Identity, OFFSET>,
            EndMethodEnumeration: EndMethodEnumeration::<Identity, OFFSET>,
            GetMethodQualifierSet: GetMethodQualifierSet::<Identity, OFFSET>,
            GetMethodOrigin: GetMethodOrigin::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClassObject as windows_core::Interface>::IID
    }
}
pub trait IWbemClientConnectionTransport_Impl: Sized + windows_core::IUnknownImpl {
    fn Open(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: Option<&IWbemContext>, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void, pcallres: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn OpenAsync(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: Option<&IWbemContext>, riid: *const windows_core::GUID, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn Cancel(&self, lflags: i32, phandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemClientConnectionTransport {}
impl IWbemClientConnectionTransport_Vtbl {
    pub const fn new<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>() -> IWbemClientConnectionTransport_Vtbl {
        unsafe extern "system" fn Open<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: core::mem::MaybeUninit<windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void, pcallres: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClientConnectionTransport_Impl::Open(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strobject), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface), core::mem::transmute_copy(&pcallres)).into()
        }
        unsafe extern "system" fn OpenAsync<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: core::mem::MaybeUninit<windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClientConnectionTransport_Impl::OpenAsync(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strobject), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn Cancel<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemClientConnectionTransport_Impl::Cancel(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&phandler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            OpenAsync: OpenAsync::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClientConnectionTransport as windows_core::Interface>::IID
    }
}
pub trait IWbemClientTransport_Impl: Sized + windows_core::IUnknownImpl {
    fn ConnectServer(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl windows_core::RuntimeName for IWbemClientTransport {}
impl IWbemClientTransport_Vtbl {
    pub const fn new<Identity: IWbemClientTransport_Impl, const OFFSET: isize>() -> IWbemClientTransport_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: IWbemClientTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: core::mem::MaybeUninit<windows_core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, lsecurityflags: i32, strauthority: core::mem::MaybeUninit<windows_core::BSTR>, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemClientTransport_Impl::ConnectServer(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strnetworkresource), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lsecurityflags), core::mem::transmute(&strauthority), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    ppnamespace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClientTransport as windows_core::Interface>::IID
    }
}
pub trait IWbemConfigureRefresher_Impl: Sized + windows_core::IUnknownImpl {
    fn AddObjectByPath(&self, pnamespace: Option<&IWbemServices>, wszpath: &windows_core::PCWSTR, lflags: i32, pcontext: Option<&IWbemContext>, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>;
    fn AddObjectByTemplate(&self, pnamespace: Option<&IWbemServices>, ptemplate: Option<&IWbemClassObject>, lflags: i32, pcontext: Option<&IWbemContext>, pprefreshable: *mut Option<IWbemClassObject>, plid: *mut i32) -> windows_core::Result<()>;
    fn AddRefresher(&self, prefresher: Option<&IWbemRefresher>, lflags: i32, plid: *mut i32) -> windows_core::Result<()>;
    fn Remove(&self, lid: i32, lflags: i32) -> windows_core::Result<()>;
    fn AddEnum(&self, pnamespace: Option<&IWbemServices>, wszclassname: &windows_core::PCWSTR, lflags: i32, pcontext: Option<&IWbemContext>, ppenum: *mut Option<IWbemHiPerfEnum>, plid: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemConfigureRefresher {}
impl IWbemConfigureRefresher_Vtbl {
    pub const fn new<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>() -> IWbemConfigureRefresher_Vtbl {
        unsafe extern "system" fn AddObjectByPath<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConfigureRefresher_Impl::AddObjectByPath(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute(&wszpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn AddObjectByTemplate<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConfigureRefresher_Impl::AddObjectByTemplate(this, windows_core::from_raw_borrowed(&pnamespace), windows_core::from_raw_borrowed(&ptemplate), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn AddRefresher<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lflags: i32, plid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConfigureRefresher_Impl::AddRefresher(this, windows_core::from_raw_borrowed(&prefresher), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn Remove<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lid: i32, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConfigureRefresher_Impl::Remove(this, core::mem::transmute_copy(&lid), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddEnum<Identity: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclassname: windows_core::PCWSTR, lflags: i32, pcontext: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConfigureRefresher_Impl::AddEnum(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute(&wszclassname), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&ppenum), core::mem::transmute_copy(&plid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddObjectByPath: AddObjectByPath::<Identity, OFFSET>,
            AddObjectByTemplate: AddObjectByTemplate::<Identity, OFFSET>,
            AddRefresher: AddRefresher::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            AddEnum: AddEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConfigureRefresher as windows_core::Interface>::IID
    }
}
pub trait IWbemConnectorLogin_Impl: Sized + windows_core::IUnknownImpl {
    fn ConnectorLogin(&self, wsznetworkresource: &windows_core::PCWSTR, wszpreferredlocale: &windows_core::PCWSTR, lflags: i32, pctx: Option<&IWbemContext>, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemConnectorLogin {}
impl IWbemConnectorLogin_Vtbl {
    pub const fn new<Identity: IWbemConnectorLogin_Impl, const OFFSET: isize>() -> IWbemConnectorLogin_Vtbl {
        unsafe extern "system" fn ConnectorLogin<Identity: IWbemConnectorLogin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszpreferredlocale: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConnectorLogin_Impl::ConnectorLogin(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectorLogin: ConnectorLogin::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConnectorLogin as windows_core::Interface>::IID
    }
}
pub trait IWbemConstructClassObject_Impl: Sized + windows_core::IUnknownImpl {
    fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPropertyOrigin(&self, wszpropertyname: &windows_core::PCWSTR, loriginindex: i32) -> windows_core::Result<()>;
    fn SetMethodOrigin(&self, wszmethodname: &windows_core::PCWSTR, loriginindex: i32) -> windows_core::Result<()>;
    fn SetServerNamespace(&self, wszserver: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemConstructClassObject {}
impl IWbemConstructClassObject_Vtbl {
    pub const fn new<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>() -> IWbemConstructClassObject_Vtbl {
        unsafe extern "system" fn SetInheritanceChain<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConstructClassObject_Impl::SetInheritanceChain(this, core::mem::transmute_copy(&lnumantecedents), core::mem::transmute_copy(&awszantecedents)).into()
        }
        unsafe extern "system" fn SetPropertyOrigin<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpropertyname: windows_core::PCWSTR, loriginindex: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConstructClassObject_Impl::SetPropertyOrigin(this, core::mem::transmute(&wszpropertyname), core::mem::transmute_copy(&loriginindex)).into()
        }
        unsafe extern "system" fn SetMethodOrigin<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethodname: windows_core::PCWSTR, loriginindex: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConstructClassObject_Impl::SetMethodOrigin(this, core::mem::transmute(&wszmethodname), core::mem::transmute_copy(&loriginindex)).into()
        }
        unsafe extern "system" fn SetServerNamespace<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszserver: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemConstructClassObject_Impl::SetServerNamespace(this, core::mem::transmute(&wszserver), core::mem::transmute(&wsznamespace)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInheritanceChain: SetInheritanceChain::<Identity, OFFSET>,
            SetPropertyOrigin: SetPropertyOrigin::<Identity, OFFSET>,
            SetMethodOrigin: SetMethodOrigin::<Identity, OFFSET>,
            SetServerNamespace: SetServerNamespace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConstructClassObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemContext_Impl: Sized + windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IWbemContext>;
    fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pvalue: *mut super::Variant::VARIANT) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
    fn SetValue(&self, wszname: &windows_core::PCWSTR, lflags: i32, pvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, wszname: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn DeleteValue(&self, wszname: &windows_core::PCWSTR, lflags: i32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemContext {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemContext_Vtbl {
    pub const fn new<Identity: IWbemContext_Impl, const OFFSET: isize>() -> IWbemContext_Vtbl {
        unsafe extern "system" fn Clone<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemContext_Impl::Clone(this) {
                Ok(ok__) => {
                    ppnewcopy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemContext_Impl::GetNames(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    pnames.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemContext_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Next<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pvalue: *mut core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemContext_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemContext_Impl::EndEnumeration(this).into()
        }
        unsafe extern "system" fn SetValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pvalue: *const core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemContext_Impl::SetValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pvalue: *mut core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemContext_Impl::GetValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteValue<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemContext_Impl::DeleteValue(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: IWbemContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemContext_Impl::DeleteAll(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            DeleteValue: DeleteValue::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemContext as windows_core::Interface>::IID
    }
}
pub trait IWbemDecoupledBasicEventProvider_Impl: Sized + IWbemDecoupledRegistrar_Impl {
    fn GetSink(&self, a_flags: i32, a_context: Option<&IWbemContext>) -> windows_core::Result<IWbemObjectSink>;
    fn GetService(&self, a_flags: i32, a_context: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl windows_core::RuntimeName for IWbemDecoupledBasicEventProvider {}
impl IWbemDecoupledBasicEventProvider_Vtbl {
    pub const fn new<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>() -> IWbemDecoupledBasicEventProvider_Vtbl {
        unsafe extern "system" fn GetSink<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_sink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemDecoupledBasicEventProvider_Impl::GetSink(this, core::mem::transmute_copy(&a_flags), windows_core::from_raw_borrowed(&a_context)) {
                Ok(ok__) => {
                    a_sink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemDecoupledBasicEventProvider_Impl::GetService(this, core::mem::transmute_copy(&a_flags), windows_core::from_raw_borrowed(&a_context)) {
                Ok(ok__) => {
                    a_service.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWbemDecoupledRegistrar_Vtbl::new::<Identity, OFFSET>(),
            GetSink: GetSink::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemDecoupledBasicEventProvider as windows_core::Interface>::IID || iid == &<IWbemDecoupledRegistrar as windows_core::Interface>::IID
    }
}
pub trait IWbemDecoupledRegistrar_Impl: Sized + windows_core::IUnknownImpl {
    fn Register(&self, a_flags: i32, a_context: Option<&IWbemContext>, a_user: &windows_core::PCWSTR, a_locale: &windows_core::PCWSTR, a_scope: &windows_core::PCWSTR, a_registration: &windows_core::PCWSTR, piunknown: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UnRegister(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemDecoupledRegistrar {}
impl IWbemDecoupledRegistrar_Vtbl {
    pub const fn new<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>() -> IWbemDecoupledRegistrar_Vtbl {
        unsafe extern "system" fn Register<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_user: windows_core::PCWSTR, a_locale: windows_core::PCWSTR, a_scope: windows_core::PCWSTR, a_registration: windows_core::PCWSTR, piunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemDecoupledRegistrar_Impl::Register(this, core::mem::transmute_copy(&a_flags), windows_core::from_raw_borrowed(&a_context), core::mem::transmute(&a_user), core::mem::transmute(&a_locale), core::mem::transmute(&a_scope), core::mem::transmute(&a_registration), windows_core::from_raw_borrowed(&piunknown)).into()
        }
        unsafe extern "system" fn UnRegister<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemDecoupledRegistrar_Impl::UnRegister(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            UnRegister: UnRegister::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemDecoupledRegistrar as windows_core::Interface>::IID
    }
}
pub trait IWbemEventConsumerProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn FindConsumer(&self, plogicalconsumer: Option<&IWbemClassObject>) -> windows_core::Result<IWbemUnboundObjectSink>;
}
impl windows_core::RuntimeName for IWbemEventConsumerProvider {}
impl IWbemEventConsumerProvider_Vtbl {
    pub const fn new<Identity: IWbemEventConsumerProvider_Impl, const OFFSET: isize>() -> IWbemEventConsumerProvider_Vtbl {
        unsafe extern "system" fn FindConsumer<Identity: IWbemEventConsumerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogicalconsumer: *mut core::ffi::c_void, ppconsumer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemEventConsumerProvider_Impl::FindConsumer(this, windows_core::from_raw_borrowed(&plogicalconsumer)) {
                Ok(ok__) => {
                    ppconsumer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindConsumer: FindConsumer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventConsumerProvider as windows_core::Interface>::IID
    }
}
pub trait IWbemEventProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn ProvideEvents(&self, psink: Option<&IWbemObjectSink>, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemEventProvider {}
impl IWbemEventProvider_Vtbl {
    pub const fn new<Identity: IWbemEventProvider_Impl, const OFFSET: isize>() -> IWbemEventProvider_Vtbl {
        unsafe extern "system" fn ProvideEvents<Identity: IWbemEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemEventProvider_Impl::ProvideEvents(this, windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideEvents: ProvideEvents::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProvider as windows_core::Interface>::IID
    }
}
pub trait IWbemEventProviderQuerySink_Impl: Sized + windows_core::IUnknownImpl {
    fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> windows_core::Result<()>;
    fn CancelQuery(&self, dwid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemEventProviderQuerySink {}
impl IWbemEventProviderQuerySink_Vtbl {
    pub const fn new<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>() -> IWbemEventProviderQuerySink_Vtbl {
        unsafe extern "system" fn NewQuery<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemEventProviderQuerySink_Impl::NewQuery(this, core::mem::transmute_copy(&dwid), core::mem::transmute_copy(&wszquerylanguage), core::mem::transmute_copy(&wszquery)).into()
        }
        unsafe extern "system" fn CancelQuery<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemEventProviderQuerySink_Impl::CancelQuery(this, core::mem::transmute_copy(&dwid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NewQuery: NewQuery::<Identity, OFFSET>,
            CancelQuery: CancelQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProviderQuerySink as windows_core::Interface>::IID
    }
}
pub trait IWbemEventProviderSecurity_Impl: Sized + windows_core::IUnknownImpl {
    fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemEventProviderSecurity {}
impl IWbemEventProviderSecurity_Vtbl {
    pub const fn new<Identity: IWbemEventProviderSecurity_Impl, const OFFSET: isize>() -> IWbemEventProviderSecurity_Vtbl {
        unsafe extern "system" fn AccessCheck<Identity: IWbemEventProviderSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemEventProviderSecurity_Impl::AccessCheck(this, core::mem::transmute_copy(&wszquerylanguage), core::mem::transmute_copy(&wszquery), core::mem::transmute_copy(&lsidlength), core::mem::transmute_copy(&psid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AccessCheck: AccessCheck::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProviderSecurity as windows_core::Interface>::IID
    }
}
pub trait IWbemEventSink_Impl: Sized + IWbemObjectSink_Impl {
    fn SetSinkSecurity(&self, lsdlength: i32, psd: *const u8) -> windows_core::Result<()>;
    fn IsActive(&self) -> windows_core::Result<()>;
    fn GetRestrictedSink(&self, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: Option<&windows_core::IUnknown>) -> windows_core::Result<IWbemEventSink>;
    fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemEventSink {}
impl IWbemEventSink_Vtbl {
    pub const fn new<Identity: IWbemEventSink_Impl, const OFFSET: isize>() -> IWbemEventSink_Vtbl {
        unsafe extern "system" fn SetSinkSecurity<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsdlength: i32, psd: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemEventSink_Impl::SetSinkSecurity(this, core::mem::transmute_copy(&lsdlength), core::mem::transmute_copy(&psd)).into()
        }
        unsafe extern "system" fn IsActive<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemEventSink_Impl::IsActive(this).into()
        }
        unsafe extern "system" fn GetRestrictedSink<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemEventSink_Impl::GetRestrictedSink(this, core::mem::transmute_copy(&lnumqueries), core::mem::transmute_copy(&awszqueries), windows_core::from_raw_borrowed(&pcallback)) {
                Ok(ok__) => {
                    ppsink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchingParameters<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemEventSink_Impl::SetBatchingParameters(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&dwmaxbuffersize), core::mem::transmute_copy(&dwmaxsendlatency)).into()
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, OFFSET>(),
            SetSinkSecurity: SetSinkSecurity::<Identity, OFFSET>,
            IsActive: IsActive::<Identity, OFFSET>,
            GetRestrictedSink: GetRestrictedSink::<Identity, OFFSET>,
            SetBatchingParameters: SetBatchingParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventSink as windows_core::Interface>::IID || iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
pub trait IWbemHiPerfEnum_Impl: Sized + windows_core::IUnknownImpl {
    fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const Option<IWbemObjectAccess>) -> windows_core::Result<()>;
    fn RemoveObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::Result<()>;
    fn GetObjects(&self, lflags: i32, unumobjects: u32, apobj: *mut Option<IWbemObjectAccess>, pureturned: *mut u32) -> windows_core::Result<()>;
    fn RemoveAll(&self, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemHiPerfEnum {}
impl IWbemHiPerfEnum_Vtbl {
    pub const fn new<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>() -> IWbemHiPerfEnum_Vtbl {
        unsafe extern "system" fn AddObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemHiPerfEnum_Impl::AddObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apids), core::mem::transmute_copy(&apobj)).into()
        }
        unsafe extern "system" fn RemoveObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemHiPerfEnum_Impl::RemoveObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apids)).into()
        }
        unsafe extern "system" fn GetObjects<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut *mut core::ffi::c_void, pureturned: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemHiPerfEnum_Impl::GetObjects(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&unumobjects), core::mem::transmute_copy(&apobj), core::mem::transmute_copy(&pureturned)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemHiPerfEnum_Impl::RemoveAll(this, core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddObjects: AddObjects::<Identity, OFFSET>,
            RemoveObjects: RemoveObjects::<Identity, OFFSET>,
            GetObjects: GetObjects::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemHiPerfEnum as windows_core::Interface>::IID
    }
}
pub trait IWbemHiPerfProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn QueryInstances(&self, pnamespace: Option<&IWbemServices>, wszclass: &windows_core::PCWSTR, lflags: i32, pctx: Option<&IWbemContext>, psink: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateRefresher(&self, pnamespace: Option<&IWbemServices>, lflags: i32) -> windows_core::Result<IWbemRefresher>;
    fn CreateRefreshableObject(&self, pnamespace: Option<&IWbemServices>, ptemplate: Option<&IWbemObjectAccess>, prefresher: Option<&IWbemRefresher>, lflags: i32, pcontext: Option<&IWbemContext>, pprefreshable: *mut Option<IWbemObjectAccess>, plid: *mut i32) -> windows_core::Result<()>;
    fn StopRefreshing(&self, prefresher: Option<&IWbemRefresher>, lid: i32, lflags: i32) -> windows_core::Result<()>;
    fn CreateRefreshableEnum(&self, pnamespace: Option<&IWbemServices>, wszclass: &windows_core::PCWSTR, prefresher: Option<&IWbemRefresher>, lflags: i32, pcontext: Option<&IWbemContext>, phiperfenum: Option<&IWbemHiPerfEnum>) -> windows_core::Result<i32>;
    fn GetObjects(&self, pnamespace: Option<&IWbemServices>, lnumobjects: i32, apobj: *mut Option<IWbemObjectAccess>, lflags: i32, pcontext: Option<&IWbemContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemHiPerfProvider {}
impl IWbemHiPerfProvider_Vtbl {
    pub const fn new<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>() -> IWbemHiPerfProvider_Vtbl {
        unsafe extern "system" fn QueryInstances<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemHiPerfProvider_Impl::QueryInstances(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute(&wszclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn CreateRefresher<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, lflags: i32, pprefresher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemHiPerfProvider_Impl::CreateRefresher(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    pprefresher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRefreshableObject<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemHiPerfProvider_Impl::CreateRefreshableObject(this, windows_core::from_raw_borrowed(&pnamespace), windows_core::from_raw_borrowed(&ptemplate), windows_core::from_raw_borrowed(&prefresher), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn StopRefreshing<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lid: i32, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemHiPerfProvider_Impl::StopRefreshing(this, windows_core::from_raw_borrowed(&prefresher), core::mem::transmute_copy(&lid), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateRefreshableEnum<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR, prefresher: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, phiperfenum: *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemHiPerfProvider_Impl::CreateRefreshableEnum(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute(&wszclass), windows_core::from_raw_borrowed(&prefresher), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext), windows_core::from_raw_borrowed(&phiperfenum)) {
                Ok(ok__) => {
                    plid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjects<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, lnumobjects: i32, apobj: *mut *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemHiPerfProvider_Impl::GetObjects(this, windows_core::from_raw_borrowed(&pnamespace), core::mem::transmute_copy(&lnumobjects), core::mem::transmute_copy(&apobj), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInstances: QueryInstances::<Identity, OFFSET>,
            CreateRefresher: CreateRefresher::<Identity, OFFSET>,
            CreateRefreshableObject: CreateRefreshableObject::<Identity, OFFSET>,
            StopRefreshing: StopRefreshing::<Identity, OFFSET>,
            CreateRefreshableEnum: CreateRefreshableEnum::<Identity, OFFSET>,
            GetObjects: GetObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemHiPerfProvider as windows_core::Interface>::IID
    }
}
pub trait IWbemLevel1Login_Impl: Sized + windows_core::IUnknownImpl {
    fn EstablishPosition(&self, wszlocalelist: &windows_core::PCWSTR, dwnumlocales: u32) -> windows_core::Result<u32>;
    fn RequestChallenge(&self, wsznetworkresource: &windows_core::PCWSTR, wszuser: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn WBEMLogin(&self, wszpreferredlocale: &windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
    fn NTLMLogin(&self, wsznetworkresource: &windows_core::PCWSTR, wszpreferredlocale: &windows_core::PCWSTR, lflags: i32, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl windows_core::RuntimeName for IWbemLevel1Login {}
impl IWbemLevel1Login_Vtbl {
    pub const fn new<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>() -> IWbemLevel1Login_Vtbl {
        unsafe extern "system" fn EstablishPosition<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlocalelist: windows_core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemLevel1Login_Impl::EstablishPosition(this, core::mem::transmute(&wszlocalelist), core::mem::transmute_copy(&dwnumlocales)) {
                Ok(ok__) => {
                    reserved.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestChallenge<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszuser: windows_core::PCWSTR, nonce: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemLevel1Login_Impl::RequestChallenge(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszuser)) {
                Ok(ok__) => {
                    nonce.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WBEMLogin<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpreferredlocale: windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemLevel1Login_Impl::WBEMLogin(this, core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&accesstoken), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    ppnamespace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NTLMLogin<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszpreferredlocale: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemLevel1Login_Impl::NTLMLogin(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    ppnamespace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EstablishPosition: EstablishPosition::<Identity, OFFSET>,
            RequestChallenge: RequestChallenge::<Identity, OFFSET>,
            WBEMLogin: WBEMLogin::<Identity, OFFSET>,
            NTLMLogin: NTLMLogin::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemLevel1Login as windows_core::Interface>::IID
    }
}
pub trait IWbemLocator_Impl: Sized + windows_core::IUnknownImpl {
    fn ConnectServer(&self, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemServices>;
}
impl windows_core::RuntimeName for IWbemLocator {}
impl IWbemLocator_Vtbl {
    pub const fn new<Identity: IWbemLocator_Impl, const OFFSET: isize>() -> IWbemLocator_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: IWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnetworkresource: core::mem::MaybeUninit<windows_core::BSTR>, struser: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, lsecurityflags: i32, strauthority: core::mem::MaybeUninit<windows_core::BSTR>, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemLocator_Impl::ConnectServer(this, core::mem::transmute(&strnetworkresource), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lsecurityflags), core::mem::transmute(&strauthority), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    ppnamespace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemLocator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemObjectAccess_Impl: Sized + IWbemClassObject_Impl {
    fn GetPropertyHandle(&self, wszpropertyname: &windows_core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> windows_core::Result<()>;
    fn WritePropertyValue(&self, lhandle: i32, lnumbytes: i32, adata: *const u8) -> windows_core::Result<()>;
    fn ReadPropertyValue(&self, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> windows_core::Result<()>;
    fn ReadDWORD(&self, lhandle: i32) -> windows_core::Result<u32>;
    fn WriteDWORD(&self, lhandle: i32, dw: u32) -> windows_core::Result<()>;
    fn ReadQWORD(&self, lhandle: i32) -> windows_core::Result<u64>;
    fn WriteQWORD(&self, lhandle: i32, pw: u64) -> windows_core::Result<()>;
    fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut windows_core::BSTR, ptype: *mut i32) -> windows_core::Result<()>;
    fn Lock(&self, lflags: i32) -> windows_core::Result<()>;
    fn Unlock(&self, lflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemObjectAccess {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemObjectAccess_Vtbl {
    pub const fn new<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>() -> IWbemObjectAccess_Vtbl {
        unsafe extern "system" fn GetPropertyHandle<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpropertyname: windows_core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectAccess_Impl::GetPropertyHandle(this, core::mem::transmute(&wszpropertyname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&plhandle)).into()
        }
        unsafe extern "system" fn WritePropertyValue<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectAccess_Impl::WritePropertyValue(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&lnumbytes), core::mem::transmute_copy(&adata)).into()
        }
        unsafe extern "system" fn ReadPropertyValue<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectAccess_Impl::ReadPropertyValue(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&lbuffersize), core::mem::transmute_copy(&plnumbytes), core::mem::transmute_copy(&adata)).into()
        }
        unsafe extern "system" fn ReadDWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemObjectAccess_Impl::ReadDWORD(this, core::mem::transmute_copy(&lhandle)) {
                Ok(ok__) => {
                    pdw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteDWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, dw: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectAccess_Impl::WriteDWORD(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn ReadQWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemObjectAccess_Impl::ReadQWORD(this, core::mem::transmute_copy(&lhandle)) {
                Ok(ok__) => {
                    pqw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteQWORD<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pw: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectAccess_Impl::WriteQWORD(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&pw)).into()
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhandle: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, ptype: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectAccess_Impl::GetPropertyInfoByHandle(this, core::mem::transmute_copy(&lhandle), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn Lock<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectAccess_Impl::Lock(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectAccess_Impl::Unlock(this, core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: IWbemClassObject_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyHandle: GetPropertyHandle::<Identity, OFFSET>,
            WritePropertyValue: WritePropertyValue::<Identity, OFFSET>,
            ReadPropertyValue: ReadPropertyValue::<Identity, OFFSET>,
            ReadDWORD: ReadDWORD::<Identity, OFFSET>,
            WriteDWORD: WriteDWORD::<Identity, OFFSET>,
            ReadQWORD: ReadQWORD::<Identity, OFFSET>,
            WriteQWORD: WriteQWORD::<Identity, OFFSET>,
            GetPropertyInfoByHandle: GetPropertyInfoByHandle::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectAccess as windows_core::Interface>::IID || iid == &<IWbemClassObject as windows_core::Interface>::IID
    }
}
pub trait IWbemObjectSink_Impl: Sized + windows_core::IUnknownImpl {
    fn Indicate(&self, lobjectcount: i32, apobjarray: *const Option<IWbemClassObject>) -> windows_core::Result<()>;
    fn SetStatus(&self, lflags: i32, hresult: windows_core::HRESULT, strparam: &windows_core::BSTR, pobjparam: Option<&IWbemClassObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemObjectSink {}
impl IWbemObjectSink_Vtbl {
    pub const fn new<Identity: IWbemObjectSink_Impl, const OFFSET: isize>() -> IWbemObjectSink_Vtbl {
        unsafe extern "system" fn Indicate<Identity: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjectcount: i32, apobjarray: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectSink_Impl::Indicate(this, core::mem::transmute_copy(&lobjectcount), core::mem::transmute_copy(&apobjarray)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hresult: windows_core::HRESULT, strparam: core::mem::MaybeUninit<windows_core::BSTR>, pobjparam: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectSink_Impl::SetStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hresult), core::mem::transmute(&strparam), windows_core::from_raw_borrowed(&pobjparam)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Indicate: Indicate::<Identity, OFFSET>, SetStatus: SetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemObjectSinkEx_Impl: Sized + IWbemObjectSink_Impl {
    fn WriteMessage(&self, uchannel: u32, strmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WriteError(&self, pobjerror: Option<&IWbemClassObject>) -> windows_core::Result<u8>;
    fn PromptUser(&self, strmessage: &windows_core::BSTR, uprompttype: u8) -> windows_core::Result<u8>;
    fn WriteProgress(&self, stractivity: &windows_core::BSTR, strcurrentoperation: &windows_core::BSTR, strstatusdescription: &windows_core::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::Result<()>;
    fn WriteStreamParameter(&self, strname: &windows_core::BSTR, vtvalue: *const super::Variant::VARIANT, ultype: u32, ulflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemObjectSinkEx {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemObjectSinkEx_Vtbl {
    pub const fn new<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>() -> IWbemObjectSinkEx_Vtbl {
        unsafe extern "system" fn WriteMessage<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uchannel: u32, strmessage: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectSinkEx_Impl::WriteMessage(this, core::mem::transmute_copy(&uchannel), core::mem::transmute(&strmessage)).into()
        }
        unsafe extern "system" fn WriteError<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjerror: *mut core::ffi::c_void, pureturned: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemObjectSinkEx_Impl::WriteError(this, windows_core::from_raw_borrowed(&pobjerror)) {
                Ok(ok__) => {
                    pureturned.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptUser<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strmessage: core::mem::MaybeUninit<windows_core::BSTR>, uprompttype: u8, pureturned: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemObjectSinkEx_Impl::PromptUser(this, core::mem::transmute(&strmessage), core::mem::transmute_copy(&uprompttype)) {
                Ok(ok__) => {
                    pureturned.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProgress<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stractivity: core::mem::MaybeUninit<windows_core::BSTR>, strcurrentoperation: core::mem::MaybeUninit<windows_core::BSTR>, strstatusdescription: core::mem::MaybeUninit<windows_core::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectSinkEx_Impl::WriteProgress(this, core::mem::transmute(&stractivity), core::mem::transmute(&strcurrentoperation), core::mem::transmute(&strstatusdescription), core::mem::transmute_copy(&upercentcomplete), core::mem::transmute_copy(&usecondsremaining)).into()
        }
        unsafe extern "system" fn WriteStreamParameter<Identity: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, vtvalue: *const core::mem::MaybeUninit<super::Variant::VARIANT>, ultype: u32, ulflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemObjectSinkEx_Impl::WriteStreamParameter(this, core::mem::transmute(&strname), core::mem::transmute_copy(&vtvalue), core::mem::transmute_copy(&ultype), core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, OFFSET>(),
            WriteMessage: WriteMessage::<Identity, OFFSET>,
            WriteError: WriteError::<Identity, OFFSET>,
            PromptUser: PromptUser::<Identity, OFFSET>,
            WriteProgress: WriteProgress::<Identity, OFFSET>,
            WriteStreamParameter: WriteStreamParameter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectSinkEx as windows_core::Interface>::IID || iid == &<IWbemObjectSink as windows_core::Interface>::IID
    }
}
pub trait IWbemObjectTextSrc_Impl: Sized + windows_core::IUnknownImpl {
    fn GetText(&self, lflags: i32, pobj: Option<&IWbemClassObject>, uobjtextformat: u32, pctx: Option<&IWbemContext>) -> windows_core::Result<windows_core::BSTR>;
    fn CreateFromText(&self, lflags: i32, strtext: &windows_core::BSTR, uobjtextformat: u32, pctx: Option<&IWbemContext>) -> windows_core::Result<IWbemClassObject>;
}
impl windows_core::RuntimeName for IWbemObjectTextSrc {}
impl IWbemObjectTextSrc_Vtbl {
    pub const fn new<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>() -> IWbemObjectTextSrc_Vtbl {
        unsafe extern "system" fn GetText<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pobj: *mut core::ffi::c_void, uobjtextformat: u32, pctx: *mut core::ffi::c_void, strtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemObjectTextSrc_Impl::GetText(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pobj), core::mem::transmute_copy(&uobjtextformat), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    strtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromText<Identity: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strtext: core::mem::MaybeUninit<windows_core::BSTR>, uobjtextformat: u32, pctx: *mut core::ffi::c_void, pnewobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemObjectTextSrc_Impl::CreateFromText(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strtext), core::mem::transmute_copy(&uobjtextformat), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    pnewobj.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            CreateFromText: CreateFromText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemObjectTextSrc as windows_core::Interface>::IID
    }
}
pub trait IWbemPath_Impl: Sized + windows_core::IUnknownImpl {
    fn SetText(&self, umode: u32, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64>;
    fn SetServer(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetServer(&self, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetNamespaceCount(&self) -> windows_core::Result<u32>;
    fn SetNamespaceAt(&self, uindex: u32, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RemoveNamespaceAt(&self, uindex: u32) -> windows_core::Result<()>;
    fn RemoveAllNamespaces(&self) -> windows_core::Result<()>;
    fn GetScopeCount(&self) -> windows_core::Result<u32>;
    fn SetScope(&self, uindex: u32, pszclass: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetScopeFromText(&self, uindex: u32, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut Option<IWbemPathKeyList>) -> windows_core::Result<()>;
    fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RemoveScope(&self, uindex: u32) -> windows_core::Result<()>;
    fn RemoveAllScopes(&self) -> windows_core::Result<()>;
    fn SetClassName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetClassName(&self, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetKeyList(&self) -> windows_core::Result<IWbemPathKeyList>;
    fn CreateClassPart(&self, lflags: i32, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteClassPart(&self, lflags: i32) -> windows_core::Result<()>;
    fn IsRelative(&self, wszmachine: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR) -> super::super::Foundation::BOOL;
    fn IsRelativeOrChild(&self, wszmachine: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL;
    fn IsLocal(&self, wszmachine: &windows_core::PCWSTR) -> super::super::Foundation::BOOL;
    fn IsSameClassName(&self, wszclass: &windows_core::PCWSTR) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IWbemPath {}
impl IWbemPath_Vtbl {
    pub const fn new<Identity: IWbemPath_Impl, const OFFSET: isize>() -> IWbemPath_Vtbl {
        unsafe extern "system" fn SetText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, umode: u32, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::SetText(this, core::mem::transmute_copy(&umode), core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn GetText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemPath_Impl::GetInfo(this, core::mem::transmute_copy(&urequestedinfo)) {
                Ok(ok__) => {
                    puresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::SetServer(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetServer<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::GetServer(this, core::mem::transmute_copy(&punamebuflength), core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn GetNamespaceCount<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemPath_Impl::GetNamespaceCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::SetNamespaceAt(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn GetNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::GetNamespaceAt(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&punamebuflength), core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn RemoveNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::RemoveNamespaceAt(this, core::mem::transmute_copy(&uindex)).into()
        }
        unsafe extern "system" fn RemoveAllNamespaces<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::RemoveAllNamespaces(this).into()
        }
        unsafe extern "system" fn GetScopeCount<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemPath_Impl::GetScopeCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pszclass: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::SetScope(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&pszclass)).into()
        }
        unsafe extern "system" fn SetScopeFromText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::SetScopeFromText(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn GetScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::GetScope(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&puclassnamebufsize), core::mem::transmute_copy(&pszclass), core::mem::transmute_copy(&pkeylist)).into()
        }
        unsafe extern "system" fn GetScopeAsText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::GetScopeAsText(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&putextbufsize), core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn RemoveScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::RemoveScope(this, core::mem::transmute_copy(&uindex)).into()
        }
        unsafe extern "system" fn RemoveAllScopes<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::RemoveAllScopes(this).into()
        }
        unsafe extern "system" fn SetClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::SetClassName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::GetClassName(this, core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn GetKeyList<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemPath_Impl::GetKeyList(this) {
                Ok(ok__) => {
                    pout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassPart<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::CreateClassPart(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn DeleteClassPart<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::DeleteClassPart(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn IsRelative<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::IsRelative(this, core::mem::transmute(&wszmachine), core::mem::transmute(&wsznamespace))
        }
        unsafe extern "system" fn IsRelativeOrChild<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::IsRelativeOrChild(this, core::mem::transmute(&wszmachine), core::mem::transmute(&wsznamespace), core::mem::transmute_copy(&lflags))
        }
        unsafe extern "system" fn IsLocal<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::IsLocal(this, core::mem::transmute(&wszmachine))
        }
        unsafe extern "system" fn IsSameClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPath_Impl::IsSameClassName(this, core::mem::transmute(&wszclass))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetText: SetText::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
            SetServer: SetServer::<Identity, OFFSET>,
            GetServer: GetServer::<Identity, OFFSET>,
            GetNamespaceCount: GetNamespaceCount::<Identity, OFFSET>,
            SetNamespaceAt: SetNamespaceAt::<Identity, OFFSET>,
            GetNamespaceAt: GetNamespaceAt::<Identity, OFFSET>,
            RemoveNamespaceAt: RemoveNamespaceAt::<Identity, OFFSET>,
            RemoveAllNamespaces: RemoveAllNamespaces::<Identity, OFFSET>,
            GetScopeCount: GetScopeCount::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            SetScopeFromText: SetScopeFromText::<Identity, OFFSET>,
            GetScope: GetScope::<Identity, OFFSET>,
            GetScopeAsText: GetScopeAsText::<Identity, OFFSET>,
            RemoveScope: RemoveScope::<Identity, OFFSET>,
            RemoveAllScopes: RemoveAllScopes::<Identity, OFFSET>,
            SetClassName: SetClassName::<Identity, OFFSET>,
            GetClassName: GetClassName::<Identity, OFFSET>,
            GetKeyList: GetKeyList::<Identity, OFFSET>,
            CreateClassPart: CreateClassPart::<Identity, OFFSET>,
            DeleteClassPart: DeleteClassPart::<Identity, OFFSET>,
            IsRelative: IsRelative::<Identity, OFFSET>,
            IsRelativeOrChild: IsRelativeOrChild::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            IsSameClassName: IsSameClassName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPath as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemPathKeyList_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn SetKey(&self, wszname: &windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetKey2(&self, wszname: &windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::Result<()>;
    fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut super::Variant::VARIANT, puapparentcimtype: *mut u32) -> windows_core::Result<()>;
    fn RemoveKey(&self, wszname: &windows_core::PCWSTR, uflags: u32) -> windows_core::Result<()>;
    fn RemoveAllKeys(&self, uflags: u32) -> windows_core::Result<()>;
    fn MakeSingleton(&self, bset: u8) -> windows_core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemPathKeyList {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemPathKeyList_Vtbl {
    pub const fn new<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>() -> IWbemPathKeyList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pukeycount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemPathKeyList_Impl::GetCount(this) {
                Ok(ok__) => {
                    pukeycount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPathKeyList_Impl::SetKey(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ucimtype), core::mem::transmute_copy(&pkeyval)).into()
        }
        unsafe extern "system" fn SetKey2<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPathKeyList_Impl::SetKey2(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ucimtype), core::mem::transmute_copy(&pkeyval)).into()
        }
        unsafe extern "system" fn GetKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPathKeyList_Impl::GetKey(this, core::mem::transmute_copy(&ukeyix), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&punamebufsize), core::mem::transmute_copy(&pszkeyname), core::mem::transmute_copy(&pukeyvalbufsize), core::mem::transmute_copy(&pkeyval), core::mem::transmute_copy(&puapparentcimtype)).into()
        }
        unsafe extern "system" fn GetKey2<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut core::mem::MaybeUninit<super::Variant::VARIANT>, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPathKeyList_Impl::GetKey2(this, core::mem::transmute_copy(&ukeyix), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&punamebufsize), core::mem::transmute_copy(&pszkeyname), core::mem::transmute_copy(&pkeyvalue), core::mem::transmute_copy(&puapparentcimtype)).into()
        }
        unsafe extern "system" fn RemoveKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPathKeyList_Impl::RemoveKey(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn RemoveAllKeys<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPathKeyList_Impl::RemoveAllKeys(this, core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn MakeSingleton<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bset: u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPathKeyList_Impl::MakeSingleton(this, core::mem::transmute_copy(&bset)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemPathKeyList_Impl::GetInfo(this, core::mem::transmute_copy(&urequestedinfo)) {
                Ok(ok__) => {
                    puresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPathKeyList_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&psztext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            SetKey2: SetKey2::<Identity, OFFSET>,
            GetKey: GetKey::<Identity, OFFSET>,
            GetKey2: GetKey2::<Identity, OFFSET>,
            RemoveKey: RemoveKey::<Identity, OFFSET>,
            RemoveAllKeys: RemoveAllKeys::<Identity, OFFSET>,
            MakeSingleton: MakeSingleton::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPathKeyList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemPropertyProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn GetProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn PutProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR, pvvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemPropertyProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemPropertyProvider_Vtbl {
    pub const fn new<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>() -> IWbemPropertyProvider_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, strclassmapping: core::mem::MaybeUninit<windows_core::BSTR>, strinstmapping: core::mem::MaybeUninit<windows_core::BSTR>, strpropmapping: core::mem::MaybeUninit<windows_core::BSTR>, pvvalue: *mut core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemPropertyProvider_Impl::GetProperty(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strlocale), core::mem::transmute(&strclassmapping), core::mem::transmute(&strinstmapping), core::mem::transmute(&strpropmapping)) {
                Ok(ok__) => {
                    pvvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutProperty<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strlocale: core::mem::MaybeUninit<windows_core::BSTR>, strclassmapping: core::mem::MaybeUninit<windows_core::BSTR>, strinstmapping: core::mem::MaybeUninit<windows_core::BSTR>, strpropmapping: core::mem::MaybeUninit<windows_core::BSTR>, pvvalue: *const core::mem::MaybeUninit<super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemPropertyProvider_Impl::PutProperty(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strlocale), core::mem::transmute(&strclassmapping), core::mem::transmute(&strinstmapping), core::mem::transmute(&strpropmapping), core::mem::transmute_copy(&pvvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            PutProperty: PutProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPropertyProvider as windows_core::Interface>::IID
    }
}
pub trait IWbemProviderIdentity_Impl: Sized + windows_core::IUnknownImpl {
    fn SetRegistrationObject(&self, lflags: i32, pprovreg: Option<&IWbemClassObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemProviderIdentity {}
impl IWbemProviderIdentity_Vtbl {
    pub const fn new<Identity: IWbemProviderIdentity_Impl, const OFFSET: isize>() -> IWbemProviderIdentity_Vtbl {
        unsafe extern "system" fn SetRegistrationObject<Identity: IWbemProviderIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pprovreg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemProviderIdentity_Impl::SetRegistrationObject(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pprovreg)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRegistrationObject: SetRegistrationObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderIdentity as windows_core::Interface>::IID
    }
}
pub trait IWbemProviderInit_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, wszuser: &windows_core::PCWSTR, lflags: i32, wsznamespace: &windows_core::PCWSTR, wszlocale: &windows_core::PCWSTR, pnamespace: Option<&IWbemServices>, pctx: Option<&IWbemContext>, pinitsink: Option<&IWbemProviderInitSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemProviderInit {}
impl IWbemProviderInit_Vtbl {
    pub const fn new<Identity: IWbemProviderInit_Impl, const OFFSET: isize>() -> IWbemProviderInit_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IWbemProviderInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuser: windows_core::PCWSTR, lflags: i32, wsznamespace: windows_core::PCWSTR, wszlocale: windows_core::PCWSTR, pnamespace: *mut core::ffi::c_void, pctx: *mut core::ffi::c_void, pinitsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemProviderInit_Impl::Initialize(this, core::mem::transmute(&wszuser), core::mem::transmute_copy(&lflags), core::mem::transmute(&wsznamespace), core::mem::transmute(&wszlocale), windows_core::from_raw_borrowed(&pnamespace), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&pinitsink)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderInit as windows_core::Interface>::IID
    }
}
pub trait IWbemProviderInitSink_Impl: Sized + windows_core::IUnknownImpl {
    fn SetStatus(&self, lstatus: i32, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemProviderInitSink {}
impl IWbemProviderInitSink_Vtbl {
    pub const fn new<Identity: IWbemProviderInitSink_Impl, const OFFSET: isize>() -> IWbemProviderInitSink_Vtbl {
        unsafe extern "system" fn SetStatus<Identity: IWbemProviderInitSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstatus: i32, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemProviderInitSink_Impl::SetStatus(this, core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetStatus: SetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderInitSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWbemQualifierSet_Impl: Sized + windows_core::IUnknownImpl {
    fn Get(&self, wszname: &windows_core::PCWSTR, lflags: i32, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>;
    fn Put(&self, wszname: &windows_core::PCWSTR, pval: *const super::Variant::VARIANT, lflavor: i32) -> windows_core::Result<()>;
    fn Delete(&self, wszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNames(&self, lflags: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> windows_core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut windows_core::BSTR, pval: *mut super::Variant::VARIANT, plflavor: *mut i32) -> windows_core::Result<()>;
    fn EndEnumeration(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWbemQualifierSet {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWbemQualifierSet_Vtbl {
    pub const fn new<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>() -> IWbemQualifierSet_Vtbl {
        unsafe extern "system" fn Get<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, lflags: i32, pval: *mut core::mem::MaybeUninit<super::Variant::VARIANT>, plflavor: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQualifierSet_Impl::Get(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn Put<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, pval: *const core::mem::MaybeUninit<super::Variant::VARIANT>, lflavor: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQualifierSet_Impl::Put(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&lflavor)).into()
        }
        unsafe extern "system" fn Delete<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQualifierSet_Impl::Delete(this, core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn GetNames<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemQualifierSet_Impl::GetNames(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    pnames.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQualifierSet_Impl::BeginEnumeration(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Next<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<super::Variant::VARIANT>, plflavor: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQualifierSet_Impl::Next(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pstrname), core::mem::transmute_copy(&pval), core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQualifierSet_Impl::EndEnumeration(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemQualifierSet as windows_core::Interface>::IID
    }
}
pub trait IWbemQuery_Impl: Sized + windows_core::IUnknownImpl {
    fn Empty(&self) -> windows_core::Result<()>;
    fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::Result<()>;
    fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::Result<()>;
    fn Parse(&self, pszlang: &windows_core::PCWSTR, pszquery: &windows_core::PCWSTR, uflags: u32) -> windows_core::Result<()>;
    fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeMemory(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemQuery {}
impl IWbemQuery_Vtbl {
    pub const fn new<Identity: IWbemQuery_Impl, const OFFSET: isize>() -> IWbemQuery_Vtbl {
        unsafe extern "system" fn Empty<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQuery_Impl::Empty(this).into()
        }
        unsafe extern "system" fn SetLanguageFeatures<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQuery_Impl::SetLanguageFeatures(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&uarraysize), core::mem::transmute_copy(&pufeatures)).into()
        }
        unsafe extern "system" fn TestLanguageFeatures<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQuery_Impl::TestLanguageFeatures(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&uarraysize), core::mem::transmute_copy(&pufeatures)).into()
        }
        unsafe extern "system" fn Parse<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlang: windows_core::PCWSTR, pszquery: windows_core::PCWSTR, uflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQuery_Impl::Parse(this, core::mem::transmute(&pszlang), core::mem::transmute(&pszquery), core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn GetAnalysis<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQuery_Impl::GetAnalysis(this, core::mem::transmute_copy(&uanalysistype), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&panalysis)).into()
        }
        unsafe extern "system" fn FreeMemory<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQuery_Impl::FreeMemory(this, core::mem::transmute_copy(&pmem)).into()
        }
        unsafe extern "system" fn GetQueryInfo<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemQuery_Impl::GetQueryInfo(this, core::mem::transmute_copy(&uanalysistype), core::mem::transmute_copy(&uinfoid), core::mem::transmute_copy(&ubufsize), core::mem::transmute_copy(&pdestbuf)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Empty: Empty::<Identity, OFFSET>,
            SetLanguageFeatures: SetLanguageFeatures::<Identity, OFFSET>,
            TestLanguageFeatures: TestLanguageFeatures::<Identity, OFFSET>,
            Parse: Parse::<Identity, OFFSET>,
            GetAnalysis: GetAnalysis::<Identity, OFFSET>,
            FreeMemory: FreeMemory::<Identity, OFFSET>,
            GetQueryInfo: GetQueryInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemQuery as windows_core::Interface>::IID
    }
}
pub trait IWbemRefresher_Impl: Sized + windows_core::IUnknownImpl {
    fn Refresh(&self, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemRefresher {}
impl IWbemRefresher_Vtbl {
    pub const fn new<Identity: IWbemRefresher_Impl, const OFFSET: isize>() -> IWbemRefresher_Vtbl {
        unsafe extern "system" fn Refresh<Identity: IWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemRefresher_Impl::Refresh(this, core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Refresh: Refresh::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemRefresher as windows_core::Interface>::IID
    }
}
pub trait IWbemServices_Impl: Sized + windows_core::IUnknownImpl {
    fn OpenNamespace(&self, strnamespace: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppworkingnamespace: *mut Option<IWbemServices>, ppresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn CancelAsyncCall(&self, psink: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn QueryObjectSink(&self, lflags: WBEM_GENERIC_FLAG_TYPE) -> windows_core::Result<IWbemObjectSink>;
    fn GetObject(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppobject: *mut Option<IWbemClassObject>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn GetObjectAsync(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn PutClass(&self, pobject: Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn PutClassAsync(&self, pobject: Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn DeleteClass(&self, strclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn DeleteClassAsync(&self, strclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateClassEnum(&self, strsuperclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn CreateClassEnumAsync(&self, strsuperclass: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn PutInstance(&self, pinst: Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn PutInstanceAsync(&self, pinst: Option<&IWbemClassObject>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn DeleteInstance(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn DeleteInstanceAsync(&self, strobjectpath: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateInstanceEnum(&self, strfilter: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn CreateInstanceEnumAsync(&self, strfilter: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecQuery(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn ExecQueryAsync(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecNotificationQuery(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>) -> windows_core::Result<IEnumWbemClassObject>;
    fn ExecNotificationQueryAsync(&self, strquerylanguage: &windows_core::BSTR, strquery: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, pinparams: Option<&IWbemClassObject>, ppoutparams: *mut Option<IWbemClassObject>, ppcallresult: *mut Option<IWbemCallResult>) -> windows_core::Result<()>;
    fn ExecMethodAsync(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: Option<&IWbemContext>, pinparams: Option<&IWbemClassObject>, presponsehandler: Option<&IWbemObjectSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemServices {}
impl IWbemServices_Vtbl {
    pub const fn new<Identity: IWbemServices_Impl, const OFFSET: isize>() -> IWbemServices_Vtbl {
        unsafe extern "system" fn OpenNamespace<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppworkingnamespace: *mut *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::OpenNamespace(this, core::mem::transmute(&strnamespace), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppworkingnamespace), core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::CancelAsyncCall(this, windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn QueryObjectSink<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, ppresponsehandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemServices_Impl::QueryObjectSink(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppresponsehandler.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::GetObject(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppobject), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn GetObjectAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::GetObjectAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn PutClass<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::PutClass(this, windows_core::from_raw_borrowed(&pobject), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn PutClassAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::PutClassAsync(this, windows_core::from_raw_borrowed(&pobject), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn DeleteClass<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::DeleteClass(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn DeleteClassAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::DeleteClassAsync(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn CreateClassEnum<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemServices_Impl::CreateClassEnum(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassEnumAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::CreateClassEnumAsync(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn PutInstance<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinst: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::PutInstance(this, windows_core::from_raw_borrowed(&pinst), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn PutInstanceAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinst: *mut core::ffi::c_void, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::PutInstanceAsync(this, windows_core::from_raw_borrowed(&pinst), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn DeleteInstance<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::DeleteInstance(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn DeleteInstanceAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::DeleteInstanceAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn CreateInstanceEnum<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilter: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemServices_Impl::CreateInstanceEnum(this, core::mem::transmute(&strfilter), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilter: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::CreateInstanceEnumAsync(this, core::mem::transmute(&strfilter), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecQuery<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, strquery: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemServices_Impl::ExecQuery(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, strquery: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::ExecQueryAsync(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, strquery: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemServices_Impl::ExecNotificationQuery(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquerylanguage: core::mem::MaybeUninit<windows_core::BSTR>, strquery: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::ExecNotificationQueryAsync(this, core::mem::transmute(&strquerylanguage), core::mem::transmute(&strquery), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecMethod<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, pinparams: *mut core::ffi::c_void, ppoutparams: *mut *mut core::ffi::c_void, ppcallresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::ExecMethod(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&pinparams), core::mem::transmute_copy(&ppoutparams), core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: IWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: core::mem::MaybeUninit<windows_core::BSTR>, strmethodname: core::mem::MaybeUninit<windows_core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut core::ffi::c_void, pinparams: *mut core::ffi::c_void, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemServices_Impl::ExecMethodAsync(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pctx), windows_core::from_raw_borrowed(&pinparams), windows_core::from_raw_borrowed(&presponsehandler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenNamespace: OpenNamespace::<Identity, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, OFFSET>,
            QueryObjectSink: QueryObjectSink::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetObjectAsync: GetObjectAsync::<Identity, OFFSET>,
            PutClass: PutClass::<Identity, OFFSET>,
            PutClassAsync: PutClassAsync::<Identity, OFFSET>,
            DeleteClass: DeleteClass::<Identity, OFFSET>,
            DeleteClassAsync: DeleteClassAsync::<Identity, OFFSET>,
            CreateClassEnum: CreateClassEnum::<Identity, OFFSET>,
            CreateClassEnumAsync: CreateClassEnumAsync::<Identity, OFFSET>,
            PutInstance: PutInstance::<Identity, OFFSET>,
            PutInstanceAsync: PutInstanceAsync::<Identity, OFFSET>,
            DeleteInstance: DeleteInstance::<Identity, OFFSET>,
            DeleteInstanceAsync: DeleteInstanceAsync::<Identity, OFFSET>,
            CreateInstanceEnum: CreateInstanceEnum::<Identity, OFFSET>,
            CreateInstanceEnumAsync: CreateInstanceEnumAsync::<Identity, OFFSET>,
            ExecQuery: ExecQuery::<Identity, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, OFFSET>,
            ExecMethod: ExecMethod::<Identity, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemServices as windows_core::Interface>::IID
    }
}
pub trait IWbemShutdown_Impl: Sized + windows_core::IUnknownImpl {
    fn Shutdown(&self, ureason: i32, umaxmilliseconds: u32, pctx: Option<&IWbemContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemShutdown {}
impl IWbemShutdown_Vtbl {
    pub const fn new<Identity: IWbemShutdown_Impl, const OFFSET: isize>() -> IWbemShutdown_Vtbl {
        unsafe extern "system" fn Shutdown<Identity: IWbemShutdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemShutdown_Impl::Shutdown(this, core::mem::transmute_copy(&ureason), core::mem::transmute_copy(&umaxmilliseconds), windows_core::from_raw_borrowed(&pctx)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Shutdown: Shutdown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemShutdown as windows_core::Interface>::IID
    }
}
pub trait IWbemStatusCodeText_Impl: Sized + windows_core::IUnknownImpl {
    fn GetErrorCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetFacilityCodeText(&self, hres: windows_core::HRESULT, localeid: u32, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IWbemStatusCodeText {}
impl IWbemStatusCodeText_Vtbl {
    pub const fn new<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>() -> IWbemStatusCodeText_Vtbl {
        unsafe extern "system" fn GetErrorCodeText<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemStatusCodeText_Impl::GetErrorCodeText(this, core::mem::transmute_copy(&hres), core::mem::transmute_copy(&localeid), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    messagetext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFacilityCodeText<Identity: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemStatusCodeText_Impl::GetFacilityCodeText(this, core::mem::transmute_copy(&hres), core::mem::transmute_copy(&localeid), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    messagetext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorCodeText: GetErrorCodeText::<Identity, OFFSET>,
            GetFacilityCodeText: GetFacilityCodeText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemStatusCodeText as windows_core::Interface>::IID
    }
}
pub trait IWbemTransport_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemTransport {}
impl IWbemTransport_Vtbl {
    pub const fn new<Identity: IWbemTransport_Impl, const OFFSET: isize>() -> IWbemTransport_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IWbemTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemTransport_Impl::Initialize(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemTransport as windows_core::Interface>::IID
    }
}
pub trait IWbemUnboundObjectSink_Impl: Sized + windows_core::IUnknownImpl {
    fn IndicateToConsumer(&self, plogicalconsumer: Option<&IWbemClassObject>, lnumobjects: i32, apobjects: *const Option<IWbemClassObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWbemUnboundObjectSink {}
impl IWbemUnboundObjectSink_Vtbl {
    pub const fn new<Identity: IWbemUnboundObjectSink_Impl, const OFFSET: isize>() -> IWbemUnboundObjectSink_Vtbl {
        unsafe extern "system" fn IndicateToConsumer<Identity: IWbemUnboundObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogicalconsumer: *mut core::ffi::c_void, lnumobjects: i32, apobjects: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWbemUnboundObjectSink_Impl::IndicateToConsumer(this, windows_core::from_raw_borrowed(&plogicalconsumer), core::mem::transmute_copy(&lnumobjects), core::mem::transmute_copy(&apobjects)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IndicateToConsumer: IndicateToConsumer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemUnboundObjectSink as windows_core::Interface>::IID
    }
}
pub trait IWbemUnsecuredApartment_Impl: Sized + IUnsecuredApartment_Impl {
    fn CreateSinkStub(&self, psink: Option<&IWbemObjectSink>, dwflags: u32, wszreserved: &windows_core::PCWSTR) -> windows_core::Result<IWbemObjectSink>;
}
impl windows_core::RuntimeName for IWbemUnsecuredApartment {}
impl IWbemUnsecuredApartment_Vtbl {
    pub const fn new<Identity: IWbemUnsecuredApartment_Impl, const OFFSET: isize>() -> IWbemUnsecuredApartment_Vtbl {
        unsafe extern "system" fn CreateSinkStub<Identity: IWbemUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, dwflags: u32, wszreserved: windows_core::PCWSTR, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWbemUnsecuredApartment_Impl::CreateSinkStub(this, windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&dwflags), core::mem::transmute(&wszreserved)) {
                Ok(ok__) => {
                    ppstub.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IUnsecuredApartment_Vtbl::new::<Identity, OFFSET>(), CreateSinkStub: CreateSinkStub::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemUnsecuredApartment as windows_core::Interface>::IID || iid == &<IUnsecuredApartment as windows_core::Interface>::IID
    }
}
