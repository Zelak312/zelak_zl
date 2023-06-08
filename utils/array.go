package utils

func Contains(arr []string, item any) bool {
	for _, i := range arr {
		if i == item {
			return true
		}
	}

	return false
}
