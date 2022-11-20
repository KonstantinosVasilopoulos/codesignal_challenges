def solution(st: String): String =
  // Return the original string if it's a palindrome already
  if isPalindrome(st) then return st

  // Starting from the end, find the longest palindrome
  var i = st.length - 1
  var j = i
  var longestPalindrome: String = String.valueOf(st.charAt(i))
  while i > 0 do
    i -= 1
    longestPalindrome += st.charAt(i)
    if isPalindrome(longestPalindrome) then j = i

  // Using the i variable, append the remaining characters to the string
  var palindrome = st
  j -= 1
  while j >= 0 do
    palindrome += st.charAt(j)
    j -= 1

  palindrome

// Checks whether a string is a palindrome
def isPalindrome(st: String): Boolean =
  var start = 0
  var end = st.length - 1
  while start < end do
    if st.charAt(start) != st.charAt(end) then
      return false

    start += 1
    end -= 1

  true

@main def testSolution: Unit =
  var palindrome = solution("abcdc");
  assert(palindrome == "abcdcba", palindrome)

  palindrome = solution("ababab")
  assert(palindrome == "abababa", palindrome)
