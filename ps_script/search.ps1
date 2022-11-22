# This ensures consistent encoding. 
function MySearcher {
	param (
		$SearchTerm, $Glob
	)

	[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

	function CreatePdfFromRgaSearch {
		param (
			$RgaString
		)
		$searchMatch = [SearchMatch]::new()
		$searchMatch.Filename = ($line | choose --field-separator : 0)
		$searchMatch.Page = ($line | choose --field-separator : 2 | choose --character-wise 5:)
		$searchMatch.LineNumber = ($line | choose --field-separator : 1)
		$searchMatch.LineContent = ($line | choose --field-separator : 3:)
		if ($searchMatch.Filename -eq "") {
			<# Action to perform if the condition is true #>
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
		$searchMatches += CreatePdfFromRgaSearch -RgaString $line
	}
	$resultString = ""
	foreach ($searchMatch in $searchMatches) {
		$res = $searchMatch.Filename, $searchMatch.Page, $searchMatch.LineNumber, $searchMatch.LineContent -Join ":"
		$res = $res.TrimEnd(":")
		$resultString += -Join $res, "`n"
	}

	$selectedPdfString = $resultString | fzf --delimiter=:  --preview-window 'down:wrap,border-left,+{3}+3/3' --preview 'rga . {1} --with-filename --no-heading --line-number | bat --highlight-line {3} --color=always' --layout=reverse

	$selectedPdf = CreatePdfFromRgaSearch -RgaString $selectedPdfString

	if ($null -eq $selectedPdf) {
		Write-Output "Closing program"
		Break
	}
	$pdfCommand = "FoxitPDFReader.exe $($selectedPdf.Filename) /A page=$($selectedPdf.Page)"
	Invoke-Expression $pdfCommand

}
MySearcher -SearchTerm $args[0] -Glob $args[1]
