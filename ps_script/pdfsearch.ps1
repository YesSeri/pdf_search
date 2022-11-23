# This ensures consistent encoding. 
function MySearcher {
	param (
		$Glob, $SearchTerm
	)

	[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

	function CreatePdfFromRgaSearch {
		param (
			$RgaString
		)
		$searchMatch = [SearchMatch]::new()
		$searchMatch.Filename = ($RgaString | choose --field-separator : 0)
		$searchMatch.Page = ($RgaString | choose --field-separator : 1 | choose --character-wise 5:)
		$searchMatch.LineNumber = ($RgaString | choose --field-separator : 2)
		$searchMatch.LineContent = ($RgaString | choose --field-separator : 3:)
		if ($searchMatch.Filename -eq "") {
			return $null
		}
		return $searchMatch
	}
	class SearchMatch {
		[string]$Filename
		[string]$Page
		[string]$LineNumber
		[string]$LineContent
	}


	$rgaCommand = "rga --color=never --ignore-case  --line-number --no-heading --path-separator / --glob $Glob $SearchTerm"

	$match = (Invoke-Expression $rgaCommand)
	$searchMatches = @()


	foreach ($line in $match) {
		$searchMatch = [SearchMatch]::new()
		$searchMatch.Filename = ($line | choose --field-separator : 0)
		$searchMatch.Page = ($line | choose --field-separator : 2 | choose --character-wise 5:)
		$searchMatch.LineNumber = ($line | choose --field-separator : 1)
		$searchMatch.LineContent = ($line | choose --field-separator : 3:)
		$searchMatches += $searchMatch
	}
	$resultString = ""
	foreach ($searchMatch in $searchMatches) {
		$res = $searchMatch.Filename, ("Page ", $searchMatch.Page -Join "" ), $searchMatch.LineNumber, $searchMatch.LineContent -Join ":"
		$res = $res.TrimEnd(":")
		$resultString += -Join $res, "`n"
	}

	$selectedPdfString = $resultString | fzf --delimiter=:  --preview-window 'down:wrap,border-left,+{3}+3/3' --preview 'rga . {1} --with-filename --no-heading --line-number | bat --highlight-line {3} --color=always' --layout=reverse

	$selectedPdf = CreatePdfFromRgaSearch -RgaString $selectedPdfString

	if ($null -eq $selectedPdf) {
		Write-Output "Closing program"
		Break
	}
	# C:\Users\Henrik\AppData\Local\SumatraPDF\SumatraPDF.exe -page 2 .\basismatnoter.pdf
	$pdfCommand = "C:\Users\Henrik\AppData\Local\SumatraPDF\SumatraPDF.exe -page $($selectedPdf.Page) $($selectedPdf.Filename)"
	Invoke-Expression $pdfCommand
}
# MySearcher -Glob $args[0] -SearchTerm $args[1]
MySearcher -Glob "assets/basismatnoter.pdf" -SearchTerm "station"
