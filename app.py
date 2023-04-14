import os
from google.cloud import storage
from io import BytesIO
from PIL import Image

# Set up credentials
os.environ["GOOGLE_APPLICATION_CREDENTIALS"] = "service_account.json"

# Set up GCS client
client = storage.Client()

# Set up bucket and directory names
bucket_name = 'ids721-prj3'
originals_dir = 'original'
resized_dir = 'resized'

# Get the bucket object
bucket = client.get_bucket(bucket_name)

# Get a list of all the files in the originals directory
blobs = bucket.list_blobs(prefix=originals_dir)

# Specify the desired height and width of the resized images
new_height = 800
new_width = 600

# Loop through each file in the directory
for blob in blobs:
    # Get the file name and extension
    file_name = os.path.basename(blob.name)
    file_extension = os.path.splitext(file_name)[1].lower()

    # Only process files that are images
    if file_extension in ['.jpg', '.jpeg', '.png', '.bmp', '.gif']:
        # Read the file from Google Cloud Storage
        file_data = BytesIO()
        blob.download_to_file(file_data)

        try:
            # Open the image using Pillow
            img = Image.open(file_data)

            # Get the image format
            img_format = img.format.lower()

            # Get the size of the image
            img_size = img.size

            # Resize the image
            resized_img = img.resize((new_height, new_width))

            # Save the resized image to Google Cloud Storage
            resized_data = BytesIO()
            resized_img.save(resized_data, format=img_format)
            resized_data.seek(0)
            new_blob = bucket.blob('{}/{}'.format(resized_dir, file_name))
            new_blob.upload_from_file(resized_data)

            print('Resized {} and uploaded to {}'.format(file_name, new_blob.name))

        except Exception as e:
            print('Error processing {}: {}'.format(file_name, e))
    else:
        print('{} is not an image file'.format(file_name))
