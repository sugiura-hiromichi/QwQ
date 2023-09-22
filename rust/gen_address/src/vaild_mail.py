import requests
import json


def is_valid_email(email):
	head = "https://emailvalidation.abstractapi.com/v1/"
	api_part = "?api_key=c0ec53d889914de3b64f3875a6a813a3"
	mail_part = "&email=" + email
	url = head + api_part + mail_part
	rsp = requests.get(url)
	return validate_email(rsp.content)


def validate_email(APIresponseObject):
	api_data = json.loads(APIresponseObject)
	print(api_data)
	if (api_data.get("deliverability") == "DELIVERABLE" and float(api_data.get("quality_score")) > 0.99):
		return True
	else:
		return False


is_valid_email("sugiura.hiromichi.2r@st.kyoto-u.ac.jp")

# valid_mails = open("valid_mails.txt", "w")
#
# try:
# 	f = open("prof_list.txt")
# 	lines = f.readlines()
# 	for line in lines:
# 		line = "mail_list/" + line.replace("\n", "")
# 		try:
# 			mail_list = open(line + ".txt")
# 			mails = mail_list.readlines()
# 			for mail in mails:
# 				mail = mail.replace("\n", "")
# 				if is_valid_email(mail):
# 					valid_mails.write(mail + "\n")
# 			valid_mails.write("\n")
# 			valid_mails.flush()
# 		except FileNotFoundError as e3:
# 			print("🫠🫠" + str(e3))
# 		finally:
# 			mail_list.close()
# 	valid_mails.write("\n")
# 	valid_mails.flush()
#
# except FileNotFoundError as e2:
# 	print("🫠" + str(e2))
# finally:
# 	f.close()
#
# valid_mails.close()
