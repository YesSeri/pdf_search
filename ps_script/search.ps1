# This ensures consistent encoding. 
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
# $glob=$args[0]
#$searchterm=$args[0]
$searchterm = "test"
$glob = "assets/*.pdf"


$fieldSeparator = ":AOSIDJ:"

$rgaCommand = "rga --color=never  --line-number --no-heading --smart-case --path-separator / --encoding UTF-8 --glob $glob --ignore-case --field-match-separator $fieldSeparator $searchterm" 

$match = (Invoke-Expression $rgaCommand)
# Write-Output $match

$result = @()
$id = 0
foreach ($line in $match) {
	$searchMatch = [SearchMatch]::new()
	$searchMatch.Id = $id
	$searchMatch.Filename, $searchMatch.Page, $searchMatch.LineContent = $line -split $fieldSeparator
	$result += $searchMatch
	$id += 1;
}

$resultString = ""
foreach ($line in $searchMatch) {
	$resultString += $searchMatch.Filename, $searchMatch.Page, $searchMatch.LineContent, "`n" -Join ":"
}

$resultString |
fzf --ansi --delimiter=":" `
	--preview-window 'down,50%,+{2}+3/3' `
	--preview 'rga "" {1} --line-number |bat --plain --color=always --highlight-line {2}' 
#echo $resultString | fzf --ansi --delimiter=":" --preview 'bat {1} --highlight-line {2} --color always' --preview-window 'up,60%,border-bottom,+{2}+3/3,~3'

class SearchMatch {
	[int]$Id
	[string]$Filename
	[string]$Page
	[string]$LineContent
}

#$result | fzf --header-lines=3

#$result | fzf --ansi --delimiter : --preview "rga --encoding UTF-8 --glob {1} . --field-match-separator :#:| choose --field-separator :#: 1| bat --plain --color=always --highlight-line {2}" --preview-window 'down:wrap,border-left,+{2}+3/3' --layout=reverse | bat -p 
# choose 0 2 --field-separator : --output-field-separator ?
# echo $result
# if ($page -eq ""){
# 	echo "Closing program"
# 	Break
# }
# $location, $page=$result.split('?')
# $page= $page -replace "Page " -replace ""
# echo "Opening $location at page $page"
# FoxitPDFReader.exe "$location" /A page="$page"
