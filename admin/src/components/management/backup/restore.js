import React, { useState } from "react";
import { Form, Input, Button, Checkbox, Upload, message } from "antd";
import axios from "axios";

const Restore = () => {
  // ---------state -----------

  const [, setLoading] = useState(false);
  const [fileList, setFileList] = useState([]);
  const [uploading, setUploading] = useState(false);
  const [confirm, setConfirm] = useState(false);

  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  // ------on change --------
  const onChange = (e) => {
    setConfirm(e.target.checked);
  };

  // ------import -----------
  const handleImport = (data, file) => {
    const formData = new FormData();
    formData.append("file", file);
    setUploading(true);
    axios
      .post(`${baseUrl}/settings/import`, {
        headers: {
          "content-type": "application/json",
          ...auth,
          responseType: "blob",
        },
        data: {
          password: data.password,
          formData,
        },
      })

      .then((res) => {
        if ((res.statusCode = 200)) {
          setLoading(true);
          setFileList();
          setUploading(false);
          message.success("Successful!");
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })
      .catch((err) => console.log(err));
  };

  //  ---------on before  upload ------------

  const props = {
    beforeUpload: (file) => {
      if (file.type !== "image/png") {
        return setFileList({ fileList: [...fileList, file] });
      } else {
        message.error(`${file.name} is not a file`);
      }
      return file.type !== "image/png" ? true : Upload.LIST_IGNORE;
    },
  };

  return (
    <React.Fragment>
      <Form {...layout} onFinish={handleImport} className="backup-container">
        <Form.Item
          className="restore-settting"
          label="File"
          rules={[
            {
              required: true,
              message: "Select the file!",
            },
          ]}
        >
          <Upload
            {...props}
            maxCount={1}
            action="https://www.mocky.io/v2/5cc8019d300000980a055e76"
            listType="picture"
            className="upload-list-inline"
          >
            <Button className="browse-buttons" size="large">
              Browse
            </Button>
          </Upload>
        </Form.Item>
        <Form.Item
          label="Password"
          name="password"
          rules={[
            {
              required: true,
              message: "Input Password!",
            },
          ]}
        >
          <Input.Password className="label-info" size="large" />
        </Form.Item>
        <Form.Item name="confirm">
          <Checkbox onChange={onChange} className="restore-checkbox">
            Confirm
          </Checkbox>
        </Form.Item>

        <Form.Item>
          <Button
            type="primary"
            htmlType="submit"
            className="button-export2"
            size="large"
            disabled={fileList.length === 0 || confirm === false}
          >
            {uploading ? "Importing" : "IMPORT"}
          </Button>
        </Form.Item>
      </Form>
    </React.Fragment>
  );
};

export default Restore;
