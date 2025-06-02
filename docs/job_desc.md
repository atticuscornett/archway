# Job Files
Job files follow JSON format and describe a backup/archive job.
Versions are designed to be backward compatible and incremental, so older versions of the job file will still work on
newer versions of the program.

(TODO: Add example job file, create job JSON schema)

## Version 1

- `job_name` (string): The name of the job.
- `uuid` (string): The UUID of the job.
- `file_behavior` (string): What to do with the files after the job is done.
  - **Values:**
    - `'copy'` (string): Copy files to the new location.
    - `'move'` (string): Move files to the new location, deleting the original files.
- `input_dirs` (Object array): List of directories to search for files to back up.
  - `path_type` (string): The type of input directory. 
    - **Values:**
      - `'library'` (string): A library directory.
      - `'custom'` (string): A custom directory.
  - `path` (string): The path to the directory.
    - **Values:** 
      - For library directories, the path is the library name.
        - Supported libraries: 'photos', 'videos', 'documents', 'music', 'downloads', 'desktop'.
      - For custom directories, the path is absolute.
- `output_dir` (string): The directory where the files will be copied/moved to.
- `output_device` (string): The name of the device where the files will be copied/moved to. If special:any, will move to any device that matches the output_dir.
- `copies` (int): When the file_behavior is 'copy', this is the number of backups to keep.
- `portable` (boolean): Whether the job is portable. If true, the job will be copied to the output device and will be
imported on other devices.
- `new_folder` (boolean): Whether to create a new folder in the output directory for the job if the output directory
  does not exist.
- `file_filters` (Object array): List of file filters to apply when searching for files to back up/archive.
  - `filter_type` (string): The type of file filter.
    - **Values:**
      - `'extension'` (string): A file extension filter.
      - `'last-used'` (string): A regular expression filter.
      - `'size'` (string): A file size filter.
    - `traits` (Object):
        - `size` (int): The size of the file in megabytes.
        - `lastused` (string): The last used date of the file.
          - **Values:**
            - `'week'` (string): The file has not been used in the last week.
            - `'2weeks'` (string): The file has not been used in the last two weeks.
            - `'month'` (string): The file has not been used in the last month.
            - `'2months'` (string): The file has not been used in the last two months.
            - `'3months'` (string): The file has not been used in the last three months.
            - `'6months'` (string): The file has not been used in the last six months.
            - `'year'` (string): The file has not been used in the last year.
        - `extensions` (string array): The file extensions to filter by, omitting the dot. (May also be a category name followed by ":special")
          - **Values:**
            - `'photos'` (string): Photos.
            - `'videos'` (string): Videos.
            - `'documents'` (string): Documents.
            - `'music'` (string): Music.
            - `'downloads'` (string): Downloads.
            - `'desktop'` (string): Desktop files.
- `triggers` (Object array): List of triggers to start the job.
  - `trigger_type` (string): The type of trigger.
    - **Values:**
      - `'time'` (string): A time-based trigger.
      - `'event'` (string): An event-based trigger.
  - `traits` (Object):
    - `time` (String array): The time to run the job.
    - `event` (string): The event to trigger the job.
      - **Values:**
        - `'device-connection'` (string): Trigger when a device is connected.
        - `'hourly'` (string): Run the job every hour.
        - `'daily'` (string): Run the job every day.
        - `'weekly'` (string): Run the job every week.
        - `'monthly'` (string): Run the job every month.
- `version` (int): The version of the job file.