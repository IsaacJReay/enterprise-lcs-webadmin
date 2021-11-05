import React, { useState } from "react";
import { Row, Col, Form, Input, Button, Checkbox, Upload, message } from "antd";
import axios from "axios";

const Restore = () => {
  // ---------state -----------

  const [loading, setLoading] = useState(false);
  const [fileList, setFileList] = useState([]);
  const [uploading, setUploading] = useState(false);
  const [confirm, setConfirm] = useState(false);

  // ------token ------

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
      .post("http://10.42.0.188:8080/private/api/settings/import", {
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
        if (res.data.operation_status === "Failed") {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        } else {
          setLoading(true);
          setFileList();
          setUploading(false);
          message.success("Successful!");
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
      <div className="container">
        <div className="container-header">
          <h1>Restore Config</h1>
        </div>
        <hr />
        <Form {...layout} onFinish={handleImport}>
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
              className="button-apply"
              size="large"
              disabled={fileList.length === 0 || confirm === false}
            >
              {uploading ? "Importing" : "Import"}
            </Button>
          </Form.Item>
        </Form>
      </div>
    </React.Fragment>
  );
};

export default Restore;
