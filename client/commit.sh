echo "Please stage all changes you want to commit.
Choose a commit type: 
0) Add file or feature
1) Bug fix 
2) Refactor 
3) Performance improvement
4) Docs or readme
5) Dependencies
6) Security 
7) Config 
8) Removal of files or dead code"

read opt

echo "Please enter a Subject line (keep imperative and less than 50 characters)"

read subj

echo "Please enter a body (Optional)"

read body

echo "Please enter a footer (Otional)"

read footer

case $opt in
	0)
	    echo "Add"
        echo $subj
        echo $body
        echo $footer
        git commit -m ":heavy_plus_sign: $subj
        
        $body

        $footer
        "
        ;;
	1)
		echo "Bug"
		;;
    2)
        echo "Refactor"
        ;;
    3)
        echo "Perform"
        ;;
	*)
		echo "Please choose a valid option"
		;;
esac