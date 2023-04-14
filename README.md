## Individual Project #3: Interact with Big Data

- Build a functional web microservice or CLI in data engineering or machine learning engineering that uses a large data platform.

________

### 1. Create a Google Cloud Storage bucket for data storage

1. Create a new Google Cloud Platform project.

<img width="206" alt="Screenshot 2023-04-14 at 2 15 58 AM" src="https://user-images.githubusercontent.com/90014065/231957928-a1388a04-4ebc-49c5-9bdd-42e06ca53b8b.png">

2. Create a new service account and download credentials as a JSON file containing a private key.

<img width="460" alt="Screenshot 2023-04-14 at 2 16 58 AM" src="https://user-images.githubusercontent.com/90014065/231958346-6b47818d-f670-4818-96ec-1086becfc71f.png">

3. Create a new Google Cloud Storage bucket and assign Storage/Object Admin permission to your account.

<img width="657" alt="Screenshot 2023-04-14 at 2 14 44 AM" src="https://user-images.githubusercontent.com/90014065/231957434-0fb60cef-2793-4826-b9f0-81b9ee17c623.png">

### 2. Upload image files to the storage 

To resize images using Google Cloud Storage, you need to upload the images you want to resize to a Google Cloud Storage bucket. The images must be in one of the following formats: '.jpg', '.jpeg', '.png', '.bmp', or '.gif'.

Once the images are uploaded, you can modify the following parameters in the app.py file:

- Credential path/name: specify the path and the name where your credentials are located.
- Bucket name: specify the name of the Google Cloud Storage bucket where the images are located.
- Original photo folder: specify the name of the folder within the bucket where the original images are located.
- Resized photo folder: specify the name of the folder within the bucket where the resized images will be saved.
- Height and width: specify the desired height and width of the resized images in pixels.

```
os.environ["GOOGLE_APPLICATION_CREDENTIALS"] = "service_account.json"

bucket_name = 'ids721-prj3'
originals_dir = 'original'
resized_dir = 'resized'

new_height = 800
new_width = 600
```

After modifying these parameters, run the `app.py` file to resize your images. The resized images will be saved in the folder you specified within the bucket and folder.

### 3. The result 

- Original images

<img width="493" alt="Screenshot 2023-04-14 at 2 31 19 AM" src="https://user-images.githubusercontent.com/90014065/231961726-4fe511f8-1849-47cf-b9c7-1e2077d4ea12.png">

![dogs1](https://user-images.githubusercontent.com/90014065/232126273-cc71f46b-e0a5-4316-b23e-cc506bb6ad24.jpeg)

- Resized images

<img width="557" alt="Screenshot 2023-04-14 at 2 35 11 AM" src="https://user-images.githubusercontent.com/90014065/231962344-60bbc450-754e-43b7-9c08-8902a1f9e095.png">

![resized_dogs1](https://user-images.githubusercontent.com/90014065/232126473-e09dc186-ea53-4ee6-9232-96a7b225b477.jpeg)



