// Copyright © 2015, Brian Vincent
// Licensed under the MIT License <LICENSE.md>
//! VSS backup interfaces
DEFINE_GUID!(IID_IVssExamineWriterMetadata, 0x902fcf7f, 0xb7fd, 0x42f8,
    0x81, 0xf1, 0xb2, 0xe4, 0x00, 0xb1, 0xe5, 0xbd);
DEFINE_GUID!(IID_IVssExamineWriterMetadataEx, 0x0c0e5ec0, 0xca44, 0x472b,
    0xb7, 0x02, 0xe6, 0x52, 0xdb, 0x1c, 0x04, 0x51);
DEFINE_GUID!(IID_IVssBackupComponents, 0x665c1d5f, 0xc218, 0x414d,
    0xa0, 0x5d, 0x7f, 0xef, 0x5f, 0x9d, 0x5c, 0x86);
DEFINE_GUID!(IID_IVssBackupComponentsEx, 0x963f03ad, 0x9e4c, 0x4a34,
    0xac, 0x15, 0xe4, 0xb6, 0x17, 0x4e, 0x50, 0x36);
STRUCT!{struct VSS_COMPONENTINFO {
    type_: ::VSS_COMPONENT_TYPE, // type is a keyword in rust
    bstrLogicalPath: ::BSTR,
    bstrComponentName: ::BSTR,
    bstrCaption: ::BSTR,
    pbIcon: *mut ::BYTE,
    cbIcon: ::UINT,
    bRestoreMetadata: bool,
    bNotifyOnBackupComplete: bool,
    bSelectable: bool,
    bSelectableForRestore: bool,
    dwComponentFlags: ::DWORD,
    cFileCount: ::UINT,
    cDatabases: ::UINT,
    cLogFiles: ::UINT,
    cDependencies: ::UINT,
}}
pub type PVSSCOMPONENTINFO = *const ::VSS_COMPONENTINFO;
RIDL!(
interface IVssWMComponent(IVssWMComponentVtbl): IUnknown(IUnknownVtbl) {
    fn GetComponentInfo(ppInfo: *mut ::PVSSCOMPONENTINFO) -> ::HRESULT,
    fn FreeComponentInfo(pInfo: ::PVSSCOMPONENTINFO) -> ::HRESULT,
    fn GetFile(iFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc) -> ::HRESULT,
    fn GetDatabaseFile(
        iDBFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetDatabaseLogFile(
        iDbLogFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetDependency(
        iDependency: ::UINT, ppDependency: *mut *mut ::IVssWMDependency
    ) -> ::HRESULT
}
);
RIDL!(
interface IVssExamineWriterMetadata(IVssExamineWriterMetadataVtbl): IUnknown(IUnknownVtbl) {
    fn GetIdentity(
        pidInstance: *mut ::VSS_ID, pidWriter: *mut ::VSS_ID,
        pbstrWriterName: *mut ::BSTR, pUsage: *mut ::VSS_USAGE_TYPE,
        pSource: *mut ::VSS_SOURCE_TYPE
    ) -> ::HRESULT,
    fn GetFileCounts(pcIncludeFiles: *mut ::UINT, pcExcludeFiles: *mut ::UINT,
        pcComponents: *mut ::UINT
    ) -> ::HRESULT,
    fn GetIncludeFile(
        iFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetExcludeFile(
        iFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetComponent(
        iComponent: ::UINT, ppComponent: *mut *mut ::IVssWMComponent
    ) -> ::HRESULT,
    fn GetRestoreMethod(
        pMethod: *mut ::VSS_RESTOREMETHOD_ENUM, pbstrService: *mut ::BSTR,
        pbstrUserProcedure: *mut ::BSTR, pwriterRestore: *mut ::VSS_WRITERRESTORE_ENUM,
        pbRebootRequired: *mut bool, pcMappings: *mut ::UINT
    ) -> ::HRESULT,
    fn GetAlternateLocationMapping(
        iMapping: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT,
    fn GetBackupSchema(pdwSchemaMask: *mut ::DWORD) -> ::HRESULT,
    fn GetDocument(pDoc: *mut ::c_void) -> ::HRESULT, //TODO IXMLDOMDocument
    fn SaveAsXML(pbstrXML: *mut ::BSTR) -> ::HRESULT,
    fn LoadFromXML(pbstrXML: *mut ::BSTR) -> ::HRESULT
}
);
RIDL!(
interface IVssExamineWriterMetadataEx(IVssExamineWriterMetadataExVtbl):
    IVssExamineWriterMetadata(IVssExamineWriterMetadataVtbl) {
    fn GetIdentityEx(
        pidInstance: *mut ::VSS_ID, pidWriter: *mut ::VSS_ID,
        pbstrWriterName: *mut ::BSTR, pbstrInstanceName: *mut ::BSTR,
        pUsage: *mut ::VSS_USAGE_TYPE, pSource: *mut ::VSS_SOURCE_TYPE
    ) -> ::HRESULT
}
);
RIDL!(
interface IVssExamineWriterMetadataEx2(IVssExamineWriterMetadataEx2Vtbl):
    IVssExamineWriterMetadataEx(IVssExamineWriterMetadataExVtbl) {
    fn GetVersion(
        pdwMajorVersion: *mut ::DWORD, pdwMinorVersion: *mut ::DWORD
    ) -> ::HRESULT,
    fn GetExcludeFromSnapshotCount(pcExcludedFromSnapshot: *mut ::UINT) -> ::HRESULT,
    fn GetExcludeFromSnapshotFile(
        iFile: ::UINT, ppFiledesc: *mut *mut ::IVssWMFiledesc
    ) -> ::HRESULT
}
);
#[repr(C)]
pub struct IVssWriterComponentsExt {
    pub lpVtbl: *const IVssWriterComponentsExtVtbl,
}
#[repr(C)]
pub struct IVssWriterComponentsExtVtbl {
    pub parent1: ::IVssWriterComponentsVtbl,
    pub parent2: ::IUnknownVtbl,
}
RIDL!(
interface IVssBackupComponents(IVssBackupComponentsVtbl): IUnknown(IUnknownVtbl) {
    fn GetWriterComponentsCount(pcComponents: *mut ::UINT) -> ::HRESULT,
    fn GetWriterComponents(
        iWriter: ::UINT, ppWriter: *mut *mut IVssWriterComponentsExt
    ) -> ::HRESULT,
    fn InitializeForBackup(bstrXML: ::BSTR) -> ::HRESULT,
    fn SetBackupState(
        bSelectComponents: bool, bBackupBootableSystemState: bool,
        backupType: ::VSS_BACKUP_TYPE, bPartialFileSupport: bool
    ) -> ::HRESULT,
    fn InitializeForRestore(bstrXML: ::BSTR) -> ::HRESULT,
    fn SetRestoreState(restoreType: ::VSS_RESTORE_TYPE) -> ::HRESULT,
    fn GatherWriterMetadata(pAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn GetWriterMetadataCount(pcWriters: *mut ::UINT) -> ::HRESULT,
    fn GetWriterMetadata(
        iWriter: ::UINT, pidInstance: *mut ::VSS_ID,
        ppMetadata: *mut *mut IVssExamineWriterMetadata
    ) -> ::HRESULT,
    fn FreeWriterMetadata() -> ::HRESULT,
    fn AddComponent(
        instanceId: ::VSS_ID, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE,
        wszLogicalPath: ::LPCWSTR, wszComponentName: ::LPCWSTR
    ) -> ::HRESULT,
    fn PrepareForBackup(ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn AbortBackup() -> ::HRESULT,
    fn GatherWriterStatus(ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn GetWriterStatusCount(pcWriters: *mut ::UINT) -> ::HRESULT,
    fn FreeWriterStatus() -> ::HRESULT,
    fn GetWriterStatus(
        iWriter: ::UINT, pidInstance: *mut ::VSS_ID, pidWriter: *mut ::VSS_ID,
        pbstrWriter: *mut ::BSTR, pnStatus: *mut ::VSS_WRITER_STATE,
        phResultFailure: *mut ::HRESULT
    ) -> ::HRESULT,
    fn SetBackupSucceeded(
        instanceId: ::VSS_ID, writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE,
        wszLogicalPath: ::LPCWSTR, wszComponentName: ::LPCWSTR, bSucceded: bool
    ) -> ::HRESULT,
    fn SetBackupOptions(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, wszBackupOptions: ::LPCWSTR
    ) -> ::HRESULT,
    fn SetSelectedForRestore(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, bSelectedForRestore: bool
    ) -> ::HRESULT,
    fn SetRestoreOptions(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, wszRestoreOptions: ::LPCWSTR
    ) -> ::HRESULT,
    fn SetAdditionalRestores(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, bAdditionalRestores: bool
    ) -> ::HRESULT,
    fn SetPreviousBackupStamp(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, wszPreviousBackupStamp: ::LPCWSTR
    ) -> ::HRESULT,
    fn SaveAsXML(pbstrXML: *mut ::BSTR) -> ::HRESULT,
    fn BackupComplete(ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn AddAlternativeLocationMapping(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, wszPath: ::LPCWSTR, wszFilespec: ::LPCWSTR, bRecursive: bool,
        wszDestination: ::LPCWSTR
    ) -> ::HRESULT,
    fn AddRestoreSubcomponent(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, wszSubComponentLogicalPath: ::LPCWSTR,
        wszSubComponentName: ::LPCWSTR, bRepair: bool
    ) -> ::HRESULT,
    fn SetFileRestoreStatus(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, status: ::VSS_FILE_RESTORE_STATUS
    ) -> ::HRESULT,
    fn AddNewTarget(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, wszPath: ::LPCWSTR, wszFileName: ::LPCWSTR, bRecursive: bool,
        wszAlternatePath: ::LPCWSTR
    ) -> ::HRESULT,
    fn SetRangesFilePath(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, iPartialFile: ::UINT, wszRangesFile: ::LPCWSTR
    ) -> ::HRESULT,
    fn PreRestore(ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn PostRestore(ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn SetContext(lContext: ::LONG) -> ::HRESULT,
    fn StartSnapshotSet(pSnapshotSetId: *mut ::VSS_ID) -> ::HRESULT,
    fn AddToSnapshotSet(
        pwszVolumeName: ::VSS_PWSZ, ProviderId: ::VSS_ID, pidSnapshot: *mut ::VSS_ID
    ) -> ::HRESULT,
    fn DoSnapshotSet(ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn DeleteSnapshots(
        SourceObjectId: ::VSS_ID, eSourceObjectType: ::VSS_OBJECT_TYPE,
        bForceDelete: ::BOOL, plDeletedSnapshots: *mut ::LONG, pNondeletedSnapshotID: *mut ::VSS_ID
    ) -> ::HRESULT,
    fn ImportSnapshots(ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn BreakSnapshotSet(SnapshotSetId: ::VSS_ID) -> ::HRESULT,
    fn GetSnapshotProperties(
        SnapshotId: ::VSS_ID,
        pProp: *mut ::VSS_SNAPSHOT_PROP
    ) -> ::HRESULT,
    fn Query(QueriedObjectId: ::VSS_ID, eQueriedObjectType: ::VSS_OBJECT_TYPE,
        eReturnedObjectsType: ::VSS_OBJECT_TYPE, ppEnum: *mut *mut ::IVssEnumObject) -> ::HRESULT,
    fn IsVolumeSupported(
        ProviderId: ::VSS_ID, pwszVolumeName: ::VSS_PWSZ,
        pbSupportedByThisProvider: *mut ::BOOL
    ) -> ::HRESULT,
    fn DisableWriterClasses(
        rgWriterClassId: *const ::VSS_ID, cClassId: ::UINT
    ) -> ::HRESULT,
    fn EnableWriterClasses(
        rgWriterClassId: *const ::VSS_ID, cClassId: ::UINT
    ) -> ::HRESULT,
    fn DisableWriterInstances(
        rgWriterInstanceId: *const ::VSS_ID, cInstanceId: ::UINT
    ) -> ::HRESULT,
    fn ExposeSnapshot(SnapshotId: ::VSS_ID, wszPathFromRoot: ::VSS_PWSZ,
        lAttributes: ::LONG, wszExpose: ::VSS_PWSZ, pwszExposed: ::VSS_PWSZ
    ) -> ::HRESULT,
    fn RevertToSnapshot(SnapshotId: ::VSS_ID, bForceDismount: ::BOOL) -> ::HRESULT,
    fn QueryRevertStatus(
        pwszVolume: ::VSS_PWSZ, ppAsync: *mut *mut ::IVssAsync
    ) -> ::HRESULT
}
);
RIDL!(
interface IVssBackupComponentsEx(IVssBackupComponentsExVtbl):
    IVssBackupComponents(IVssBackupComponentsVtbl) {
    fn GetWriterMetadataEx(
        iWriter: ::UINT, pidInstance: *mut ::VSS_ID,
        ppMetadata: *mut *mut ::IVssExamineWriterMetadataEx
    ) -> ::HRESULT,
    fn SetSelectedForRestoreEx(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, bSelectedForRestore: bool, instanceId: ::VSS_ID
    ) -> ::HRESULT
}
);
RIDL!(
interface IVssBackupComponentsEx2(IVssBackupComponentsEx2Vtbl):
    IVssBackupComponentsEx(IVssBackupComponentsExVtbl) {
    fn UnexposeSnapshot(snapshotId: ::VSS_ID) -> ::HRESULT,
    fn SetAuthoritativeRestore(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, bAuth: bool
    ) -> ::HRESULT,
    fn SetRollForward(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, rollType: ::VSS_ROLLFORWARD_TYPE,
        wszRollForwardPoint: ::LPCWSTR
    ) -> ::HRESULT,
    fn SetRestoreName(
        writerId: ::VSS_ID, ct: ::VSS_COMPONENT_TYPE, wszLogicalPath: ::LPCWSTR,
        wszComponentName: ::LPCWSTR, wszRestoreName: ::LPCWSTR
    ) -> ::HRESULT,
    fn BreakSnapshotSetEx(
        SnapshotSetID: ::VSS_ID, dwBreakFlags: ::DWORD, ppAsync: *mut *mut ::IVssAsync
    ) -> ::HRESULT,
    fn PreFastRecovery(
        SnapshotSetID: ::VSS_ID, dwPreFastRecoveryFlags: ::DWORD,
        ppAsync: *mut *mut ::IVssAsync
    ) -> ::HRESULT,
    fn FastRecovery(
        SnapshotSetID: ::VSS_ID, dwFastRecoveryFlags: ::DWORD,
        ppAsync: *mut *mut ::IVssAsync
    ) -> ::HRESULT
}
);
RIDL!(
interface IVssBackupComponentsEx3(IVssBackupComponentsEx3Vtbl):
    IVssBackupComponentsEx2(IVssBackupComponentsEx2Vtbl) {
    fn GetWriterStatusEx(
        iWriter: ::UINT, pidInstance: *mut ::VSS_ID, pidWriter: *mut ::VSS_ID,
        pbstrWriter: *mut ::BSTR, pnStatus: *mut ::VSS_WRITER_STATE,
        phrFailureWriter: *mut ::HRESULT, phrApplication: *mut ::HRESULT,
        pbstrApplicationMessage: *mut ::BSTR
    ) -> ::HRESULT,
    fn AddSnapshotToRecoverySet(
        snapshotId: ::VSS_ID, dwFlags: ::DWORD, pwszDestinationVolume: ::VSS_PWSZ
    ) -> ::HRESULT,
    fn RecoverSet(dwFlags: ::DWORD, ppAsync: *mut *mut ::IVssAsync) -> ::HRESULT,
    fn GetSessionId(idSession: *mut ::VSS_ID) -> ::HRESULT
}
);
RIDL!(
interface IVssBackupComponentsEx4(IVssBackupComponentsEx4Vtbl):
    IVssBackupComponentsEx3(IVssBackupComponentsEx3Vtbl) {
    fn GetRootAndLogicalPrefixPaths(
        pwszFilePath: ::VSS_PWSZ, ppwszRootPath: *mut ::VSS_PWSZ,
        ppwszLogicalPrefix: *mut ::VSS_PWSZ, bNormalizeFQDNforRootPath: ::BOOL
    ) -> ::HRESULT
}
);
pub const VSS_SW_BOOTABLE_STATE: ::DWORD = 1;
