; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{D127781F-0661-414B-9745-4F61ECCC73B1}
AppName=VRInsight CDU II MSFS Driver
AppVersion=1.0.0
;AppVerName=VRInsight CDU II MSFS Driver 1.0.0
AppPublisher=callebstrom
AppPublisherURL=https://github.com/callebstrom/vrinsight-cdu-ii-msfs-driver
AppSupportURL=https://github.com/callebstrom/vrinsight-cdu-ii-msfs-driver
AppUpdatesURL=https://github.com/callebstrom/vrinsight-cdu-ii-msfs-driver
DefaultDirName={autopf}\VRInsight CDU II MSFS Driver
DisableProgramGroupPage=yes
LicenseFile=LICENSE
; Remove the following line to run in administrative install mode (install for all users.)
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputBaseFilename=mysetup
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "target/release/vrinsight-cdu-ii-msfs-driver.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "CHANGELOG.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "LICENSE"; DestDir: "{app}"; Flags: ignoreversion
Source: "SimConnect.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "keymap.yaml"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{autoprograms}\VRInsight CDU II MSFS Driver"; Filename: "{app}\vrinsight-cdu-ii-msfs-driver.exe"
Name: "{autodesktop}\VRInsight CDU II MSFS Driver"; Filename: "{app}\vrinsight-cdu-ii-msfs-driver.exe"; Tasks: desktopicon

