[CmdletBinding(DefaultParameterSetName = 'Register')]
param(
    [Parameter(ParameterSetName = 'Register')]
    [string]$Executable = (Join-Path $PSScriptRoot 'pdfree.exe'),
    [Parameter(Mandatory, ParameterSetName = 'Unregister')]
    [switch]$Unregister
)
$ErrorActionPreference = 'Stop'

$registry = [Microsoft.Win32.Registry]::CurrentUser
$appName = 'PDFree'
$progId = 'PDFree.PDF'
$appKey = 'Software\Classes\Applications\pdfree.exe'
$progIdKey = "Software\Classes\$progId"
$capabilities = 'Software\PDFree\Capabilities'
$registeredApps = 'Software\RegisteredApplications'
$openWith = 'Software\Classes\.pdf\OpenWithProgids'

function Set-RegistrationValue($Path, $Name, $Value) {
    $key = $registry.CreateSubKey($Path)
    try { $key.SetValue($Name, $Value, [Microsoft.Win32.RegistryValueKind]::String) }
    finally { $key.Dispose() }
}

function Remove-RegistrationValue($Path, $Name) {
    $key = $registry.OpenSubKey($Path, $true)
    if ($null -ne $key) {
        try { $key.DeleteValue($Name, $false) }
        finally { $key.Dispose() }
    }
}

if ($Unregister) {
    Remove-RegistrationValue $registeredApps $appName
    Remove-RegistrationValue $openWith $progId
    foreach ($key in @($appKey, $progIdKey, $capabilities)) {
        $registry.DeleteSubKeyTree($key, $false)
    }
} else {
    $exe = Get-Item -LiteralPath $Executable
    if ($exe -isnot [IO.FileInfo] -or $exe.Name -ne 'pdfree.exe') {
        throw 'Choose an existing pdfree.exe in the folder where you plan to keep PDFree.'
    }

    $command = '"{0}" "%1"' -f $exe.FullName
    Set-RegistrationValue "$appKey\shell\open\command" '' $command
    Set-RegistrationValue $appKey 'FriendlyAppName' $appName
    Set-RegistrationValue "$appKey\SupportedTypes" '.pdf' ''
    Set-RegistrationValue $progIdKey '' 'PDF document'
    Set-RegistrationValue "$progIdKey\shell\open\command" '' $command
    Set-RegistrationValue $capabilities 'ApplicationName' $appName
    Set-RegistrationValue $capabilities 'ApplicationDescription' 'Open and edit PDF documents with PDFree.'
    Set-RegistrationValue "$capabilities\FileAssociations" '.pdf' $progId
    Set-RegistrationValue $openWith $progId ''
    Set-RegistrationValue $registeredApps $appName $capabilities
}

if (-not ('PDFree.ShellAssociation' -as [type])) {
    Add-Type -Namespace PDFree -Name ShellAssociation -MemberDefinition @"
[System.Runtime.InteropServices.DllImport("shell32.dll", ExactSpelling = true)]
public static extern void SHChangeNotify(int eventId, uint flags, System.IntPtr item1, System.IntPtr item2);
"@
}
# SHCNE_ASSOCCHANGED with SHCNF_IDLIST refreshes Explorer's association cache.
[PDFree.ShellAssociation]::SHChangeNotify(0x08000000, 0, [IntPtr]::Zero, [IntPtr]::Zero)

if ($Unregister) {
    Write-Host 'Removed PDFree registration for this Windows user.'
} else {
    Write-Host 'PDFree is registered for .pdf. Select it in Open with or Settings > Apps > Default apps.'
}
