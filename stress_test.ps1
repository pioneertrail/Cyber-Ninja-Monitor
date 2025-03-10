# CPU Stress Function - Maximum intensity version
function Start-CPUStress {
    $numCores = (Get-WmiObject -Class Win32_Processor).NumberOfLogicalProcessors
    foreach ($core in 1..$numCores) {
        Start-Job -ScriptBlock {
            # Disable garbage collection to increase CPU load
            [System.GC]::Collect()
            [System.GC]::WaitForPendingFinalizers()
            
            while ($true) {
                # Heavy mathematical operations
                $matrix = New-Object 'double[,]' 100,100
                for ($i = 0; $i -lt 100; $i++) {
                    for ($j = 0; $j -lt 100; $j++) {
                        $matrix[$i,$j] = [Math]::Sin($i) * [Math]::Cos($j) * [Math]::Sqrt([Math]::Pow($i, 2) + [Math]::Pow($j, 2))
                    }
                }
                
                # Busy-wait loop
                $spin = 0
                for ($i = 0; $i -lt 1000000; $i++) {
                    $spin++
                }
            }
        }
    }
}

# Memory Stress Function - More aggressive
function Start-MemoryStress {
    Start-Job -ScriptBlock {
        $lists = @()
        while ($true) {
            # Create multiple large arrays simultaneously
            1..5 | ForEach-Object {
                $list = New-Object System.Collections.ArrayList
                # Allocate 500MB per iteration
                $list.Add("A" * (500 * 1024 * 1024))
                $lists += $list
            }
            Start-Sleep -Milliseconds 1
        }
    }
}

# Network Stress Function - More parallel requests
function Start-NetworkStress {
    # Increase to 10 parallel network jobs
    1..10 | ForEach-Object {
        Start-Job -ScriptBlock {
            while ($true) {
                try {
                    # Make multiple requests in parallel
                    $urls = @(
                        "http://www.google.com",
                        "http://www.bing.com",
                        "http://www.yahoo.com"
                    )
                    $urls | ForEach-Object {
                        Start-Job -ScriptBlock {
                            param($url)
                            Invoke-WebRequest -Uri $url -UseBasicParsing | Out-Null
                        } -ArgumentList $_ | Out-Null
                    }
                } catch {
                    # Ignore errors and continue
                }
                Start-Sleep -Milliseconds 10
            }
        }
    }
}

Write-Host "Starting MAXIMUM INTENSITY stress tests..."

# Start CPU stress on all cores
Start-CPUStress

# Start multiple memory stress jobs
1..3 | ForEach-Object { Start-Job -ScriptBlock ${function:Start-MemoryStress} }

# Start network stress
Start-NetworkStress

Write-Host "System under maximum load! Press Enter to stop all stress tests..."
Read-Host

Get-Job | Stop-Job
Get-Job | Remove-Job

Write-Host "Stress test stopped." 